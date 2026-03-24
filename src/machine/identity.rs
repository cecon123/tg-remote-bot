#![allow(dead_code)]

use anyhow::Result;
use sha2::{Digest, Sha256};

pub fn machine_id() -> Result<String> {
    let hostname = hostname()?;
    let mac = primary_mac()?;
    let raw = format!("{hostname}{mac}");
    let hash = Sha256::digest(raw.as_bytes());
    Ok(hex::encode(&hash[..4]))
}

pub fn machine_label() -> Result<String> {
    let id = machine_id()?;
    let hostname = hostname()?;
    Ok(format!("{id} @ {hostname}"))
}

fn hostname() -> Result<String> {
    let mut buf = [0u16; 256];
    let mut len = buf.len() as u32;
    let ok = unsafe {
        windows_sys::Win32::System::WindowsProgramming::GetComputerNameW(buf.as_mut_ptr(), &mut len)
    };
    if ok == 0 {
        anyhow::bail!("GetComputerNameW failed");
    }
    Ok(String::from_utf16_lossy(&buf[..len as usize]))
}

fn primary_mac() -> Result<String> {
    let mut buf_len: u32 = 15_000;
    let mut buf: Vec<u8> = vec![0u8; buf_len as usize];

    loop {
        let ret = unsafe {
            windows_sys::Win32::NetworkManagement::IpHelper::GetAdaptersAddresses(
                0,
                0x100,
                std::ptr::null(),
                buf.as_mut_ptr()
                    as *mut windows_sys::Win32::NetworkManagement::IpHelper::IP_ADAPTER_ADDRESSES_LH,
                &mut buf_len,
            )
        };

        if ret == 111 {
            buf.resize(buf_len as usize, 0);
            continue;
        }
        if ret != 0 {
            anyhow::bail!("GetAdaptersAddresses failed: {ret}");
        }
        break;
    }

    let mut current = buf.as_ptr()
        as *const windows_sys::Win32::NetworkManagement::IpHelper::IP_ADAPTER_ADDRESSES_LH;
    while !current.is_null() {
        let addr = unsafe { &*current };
        if addr.PhysicalAddressLength == 6 {
            let mac = unsafe { std::slice::from_raw_parts(addr.PhysicalAddress.as_ptr(), 6) };
            let mac_str = mac
                .iter()
                .map(|b| format!("{b:02X}"))
                .collect::<Vec<_>>()
                .join(":");
            return Ok(mac_str);
        }
        current = addr.Next;
    }

    anyhow::bail!("no adapter with MAC found");
}
