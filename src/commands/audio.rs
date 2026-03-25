use anyhow::{Context, Result};
use teloxide::prelude::*;
use teloxide::types::{ChatId, MessageId};

use crate::bot::md;

const TYPE_SOURCE: &str = r#"
using System;
using System.Runtime.InteropServices;

public class AudioHelper
{
    [DllImport("winmm.dll")]
    static extern uint waveOutGetVolume(IntPtr h, out uint v);

    [DllImport("winmm.dll")]
    static extern uint waveOutSetVolume(IntPtr h, uint v);

    [DllImport("ole32.dll")]
    static extern int CoCreateInstance(ref Guid rclsid, IntPtr pUnkOuter, uint dwClsContext, ref Guid riid, out IntPtr ppv);

    static Guid CLSID_MMDeviceEnumerator = new Guid("BCDE0395-E52F-467C-8E3D-C4579291692E");
    static Guid IID_IMMDeviceEnumerator = new Guid("A95664D2-9614-4F35-A746-DE8DB63617E6");
    static Guid IID_IAudioEndpointVolume = new Guid("5CDF2C82-841E-4546-9722-0CF74078229A");

    static IntPtr GetEndpointVolume()
    {
        IntPtr pEnum = IntPtr.Zero;
        IntPtr pDevice = IntPtr.Zero;
        IntPtr pUnk = IntPtr.Zero;
        try
        {
            CoCreateInstance(ref CLSID_MMDeviceEnumerator, IntPtr.Zero, 1, ref IID_IMMDeviceEnumerator, out pEnum);
            if (pEnum == IntPtr.Zero) throw new Exception("CoCreateInstance failed");

            // vtable[4] = GetDefaultAudioEndpoint
            IntPtr vtEnum = Marshal.ReadIntPtr(pEnum);
            IntPtr pfnGetDefault = Marshal.ReadIntPtr(vtEnum, 4 * IntPtr.Size);
            var getDevice = (GetDefaultAudioEndpoint)Marshal.GetDelegateForFunctionPointer(pfnGetDefault, typeof(GetDefaultAudioEndpoint));
            int hr = getDevice(pEnum, 0, 0, out pDevice);
            if (hr != 0 || pDevice == IntPtr.Zero) throw new Exception("GetDefaultAudioEndpoint hr=" + hr);

            // vtable[3] = Activate
            IntPtr vtDevice = Marshal.ReadIntPtr(pDevice);
            IntPtr pfnActivate = Marshal.ReadIntPtr(vtDevice, 3 * IntPtr.Size);
            var activate = (ActivateDevice)Marshal.GetDelegateForFunctionPointer(pfnActivate, typeof(ActivateDevice));
            hr = activate(pDevice, ref IID_IAudioEndpointVolume, 0x10000 /*CLSCTX_ALL*/, IntPtr.Zero, out pUnk);
            if (hr != 0 || pUnk == IntPtr.Zero) throw new Exception("Activate hr=" + hr);

            return pUnk;
        }
        finally
        {
            if (pDevice != IntPtr.Zero) Marshal.Release(pDevice);
            if (pEnum != IntPtr.Zero) Marshal.Release(pEnum);
        }
    }

    delegate int GetDefaultAudioEndpoint(IntPtr self, int dataFlow, int role, out IntPtr device);
    delegate int ActivateDevice(IntPtr self, ref Guid iid, int ctx, IntPtr activationParams, out IntPtrppv);

    public static void SetMute(bool mute)
    {
        IntPtr ep = GetEndpointVolume();
        try
        {
            // vtable[14] = SetMute
            IntPtr vt = Marshal.ReadIntPtr(ep);
            IntPtr pfn = Marshal.ReadIntPtr(vt, 14 * IntPtr.Size);
            var fn = (SetMuteDelegate)Marshal.GetDelegateForFunctionPointer(pfn, typeof(SetMuteDelegate));
            Guid empty = Guid.Empty;
            fn(ep, mute, ref empty);
        }
        finally { Marshal.Release(ep); }
    }

    delegate int SetMuteDelegate(IntPtr self, bool mute, ref Guid eventContext);

    public static void SetVolume(int level)
    {
        level = Math.Max(0, Math.Min(100, level));
        uint val = (uint)(level * 0xFFFF / 100);
        waveOutSetVolume(IntPtr.Zero, (val << 16) | val);
    }

    public static int GetVolume()
    {
        uint val;
        waveOutGetVolume(IntPtr.Zero, out val);
        return (int)(val & 0xFFFF) * 100 / 0xFFFF;
    }
}
"#;

fn run_powershell(script: &str) -> Result<()> {
    let output = std::process::Command::new("powershell")
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            script,
        ])
        .output()
        .context("cannot run powershell")?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("PowerShell failed: {}", stderr.trim());
    }
    Ok(())
}

fn do_mute() -> Result<()> {
    run_powershell(&format!(
        r#"Add-Type -TypeDefinition @'
{TYPE_SOURCE}
'@; [AudioHelper]::SetMute($true)"#
    ))
}

fn do_unmute() -> Result<()> {
    run_powershell(&format!(
        r#"Add-Type -TypeDefinition @'
{TYPE_SOURCE}
'@; [AudioHelper]::SetMute($false)"#
    ))
}

fn do_set_volume(level: u8) -> Result<()> {
    let level = level.min(100);
    run_powershell(&format!(
        r#"Add-Type -TypeDefinition @'
{TYPE_SOURCE}
'@; [AudioHelper]::SetVolume({level})"#
    ))
}

pub async fn mute(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    tokio::task::spawn_blocking(do_mute).await??;
    md::send(bot, chat_id, reply_to, "🔇 Đã tắt âm".to_string()).await
}

pub async fn unmute(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    tokio::task::spawn_blocking(do_unmute).await??;
    md::send(bot, chat_id, reply_to, "🔊 Đã bật âm".to_string()).await
}

pub async fn set_volume(bot: &Bot, chat_id: ChatId, reply_to: MessageId, level: u8) -> Result<()> {
    let level = level.min(100);
    tokio::task::spawn_blocking(move || do_set_volume(level)).await??;
    md::send(bot, chat_id, reply_to, format!("🔊 Âm lượng: {}\\%", level)).await
}
