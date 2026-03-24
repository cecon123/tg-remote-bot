use std::path::Path;

use anyhow::Result;

struct Handle(windows_sys::Win32::Foundation::HANDLE);

impl Drop for Handle {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { windows_sys::Win32::Foundation::CloseHandle(self.0) };
        }
    }
}

pub fn is_system_session() -> bool {
    unsafe {
        let id = windows_sys::Win32::System::RemoteDesktop::WTSGetActiveConsoleSessionId();
        id == u32::MAX
    }
}

fn get_user_primary_token() -> Result<Handle> {
    use windows_sys::Win32::Security as sec;
    use windows_sys::Win32::System::RemoteDesktop as wts;

    unsafe {
        let session_id = wts::WTSGetActiveConsoleSessionId();
        if session_id == u32::MAX {
            anyhow::bail!("no active user session");
        }

        let mut user_token: windows_sys::Win32::Foundation::HANDLE = std::ptr::null_mut();
        if wts::WTSQueryUserToken(session_id, &mut user_token) == 0 {
            anyhow::bail!(
                "WTSQueryUserToken failed: {}",
                std::io::Error::last_os_error()
            );
        }
        let _user = Handle(user_token);

        let mut primary: windows_sys::Win32::Foundation::HANDLE = std::ptr::null_mut();
        if sec::DuplicateTokenEx(
            user_token,
            sec::TOKEN_ALL_ACCESS,
            std::ptr::null(),
            sec::SecurityImpersonation,
            sec::TokenPrimary,
            &mut primary,
        ) == 0
        {
            anyhow::bail!(
                "DuplicateTokenEx failed: {}",
                std::io::Error::last_os_error()
            );
        }

        Ok(Handle(primary))
    }
}

fn build_cmdline(exe: &Path, args: &[String]) -> Vec<u16> {
    let mut cmd = format!("\"{}\"", exe.display());
    for a in args {
        cmd.push(' ');
        if a.contains(' ') {
            cmd.push('"');
            cmd.push_str(a);
            cmd.push('"');
        } else {
            cmd.push_str(a);
        }
    }
    cmd.encode_utf16().chain(std::iter::once(0)).collect()
}

pub fn run_in_user_session(exe: &Path, args: Vec<String>, wait_ms: u32) -> Result<u32> {
    use windows_sys::Win32::System::Threading as proc;

    unsafe {
        let primary = get_user_primary_token()?;
        let mut si: proc::STARTUPINFOW = std::mem::zeroed();
        si.cb = std::mem::size_of::<proc::STARTUPINFOW>() as u32;
        let mut pi: proc::PROCESS_INFORMATION = std::mem::zeroed();
        let mut cmd = build_cmdline(exe, &args);

        let ok = proc::CreateProcessAsUserW(
            primary.0,
            std::ptr::null(),
            cmd.as_mut_ptr(),
            std::ptr::null(),
            std::ptr::null(),
            0,
            proc::CREATE_NO_WINDOW,
            std::ptr::null(),
            std::ptr::null(),
            &si,
            &mut pi,
        );
        if ok == 0 {
            anyhow::bail!(
                "CreateProcessAsUserW failed: {}",
                std::io::Error::last_os_error()
            );
        }

        let proc_h = Handle(pi.hProcess);
        let thread_h = Handle(pi.hThread);

        if wait_ms > 0 {
            proc::WaitForSingleObject(proc_h.0, wait_ms);
        }

        let mut exit_code: u32 = 0;
        proc::GetExitCodeProcess(proc_h.0, &mut exit_code);
        drop(thread_h);

        Ok(exit_code)
    }
}

pub fn capture_in_user_session(
    exe: &Path,
    args: Vec<String>,
    wait_ms: u32,
) -> Result<(u32, Vec<u8>)> {
    use windows_sys::Win32::System::Pipes;
    use windows_sys::Win32::System::Threading as proc;

    unsafe {
        let primary = get_user_primary_token()?;

        let mut stdout_read: windows_sys::Win32::Foundation::HANDLE = std::ptr::null_mut();
        let mut stdout_write: windows_sys::Win32::Foundation::HANDLE = std::ptr::null_mut();
        let sa = windows_sys::Win32::Security::SECURITY_ATTRIBUTES {
            nLength: std::mem::size_of::<windows_sys::Win32::Security::SECURITY_ATTRIBUTES>()
                as u32,
            lpSecurityDescriptor: std::ptr::null_mut(),
            bInheritHandle: 1,
        };
        Pipes::CreatePipe(&mut stdout_read, &mut stdout_write, &sa, 0);

        let mut si: proc::STARTUPINFOW = std::mem::zeroed();
        si.cb = std::mem::size_of::<proc::STARTUPINFOW>() as u32;
        si.dwFlags = 0x100; // STARTF_USESTDHANDLES
        si.hStdOutput = stdout_write;
        si.hStdError = stdout_write;
        let mut pi: proc::PROCESS_INFORMATION = std::mem::zeroed();
        let mut cmd = build_cmdline(exe, &args);

        let ok = proc::CreateProcessAsUserW(
            primary.0,
            std::ptr::null(),
            cmd.as_mut_ptr(),
            std::ptr::null(),
            std::ptr::null(),
            1, // bInheritHandles
            proc::CREATE_NO_WINDOW,
            std::ptr::null(),
            std::ptr::null(),
            &si,
            &mut pi,
        );

        // Close our copy of write end regardless of success
        windows_sys::Win32::Foundation::CloseHandle(stdout_write);

        if ok == 0 {
            windows_sys::Win32::Foundation::CloseHandle(stdout_read);
            anyhow::bail!(
                "CreateProcessAsUserW failed: {}",
                std::io::Error::last_os_error()
            );
        }

        let proc_h = Handle(pi.hProcess);
        let thread_h = Handle(pi.hThread);

        if wait_ms > 0 {
            proc::WaitForSingleObject(proc_h.0, wait_ms);
        }

        let mut exit_code: u32 = 0;
        proc::GetExitCodeProcess(proc_h.0, &mut exit_code);
        drop(thread_h);

        let mut output = Vec::new();
        let mut buf = [0u8; 4096];
        loop {
            let mut read: u32 = 0;
            let ok = windows_sys::Win32::Storage::FileSystem::ReadFile(
                stdout_read,
                buf.as_mut_ptr(),
                buf.len() as u32,
                &mut read,
                std::ptr::null_mut(),
            );
            if ok == 0 || read == 0 {
                break;
            }
            output.extend_from_slice(&buf[..read as usize]);
        }
        windows_sys::Win32::Foundation::CloseHandle(stdout_read);

        Ok((exit_code, output))
    }
}
