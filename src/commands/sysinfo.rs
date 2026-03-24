use anyhow::Result;
use sysinfo::{Disks, Networks, System};
use teloxide::prelude::*;
use teloxide::types::{ChatId, MessageId};

use crate::bot::md;

pub async fn sysinfo(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let os = System::long_os_version().unwrap_or_default();
    let kernel = System::kernel_version().unwrap_or_default();
    let host = System::host_name().unwrap_or_default();
    let cpu_count = sys.cpus().len();
    let cpu_brand = sys
        .cpus()
        .first()
        .map(|c| c.brand().to_string())
        .unwrap_or_default();
    let mem_total = sys.total_memory() / 1024 / 1024;
    let mem_used = sys.used_memory() / 1024 / 1024;
    let mem_pct = if mem_total > 0 {
        mem_used * 100 / mem_total
    } else {
        0
    };
    let swap_total = sys.total_swap() / 1024 / 1024;
    let swap_used = sys.used_swap() / 1024 / 1024;

    let mut text = format!(
        "*ℹ️ System Info*\n\n\
        🖥️ Host: `{}`\n\
        💿 OS: {}\n\
        🔧 Kernel: {}\n\
        🧠 CPU: {} \\({cpu_count} cores\\)\n\
        📊 RAM: {mem_used}/{mem_total} MB \\({mem_pct}%\\)\n\
        💾 Swap: {swap_used}/{swap_total} MB\n",
        md::escape(&host),
        md::escape(&os),
        md::escape(&kernel),
        md::escape(&cpu_brand),
    );

    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        let total = disk.total_space() / 1024 / 1024 / 1024;
        let used = total - disk.available_space() / 1024 / 1024 / 1024;
        let pct = if total > 0 { used * 100 / total } else { 0 };
        text.push_str(&format!(
            "📁 {}:{}/{} GB \\({pct}%\\)\n",
            md::escape(&disk.mount_point().display().to_string()),
            used,
            total,
        ));
    }

    let networks = Networks::new_with_refreshed_list();
    for (name, data) in &networks {
        text.push_str(&format!(
            "🌐 {}:{} MB {} MB\n",
            md::escape(name),
            data.total_received() / 1024 / 1024,
            data.total_transmitted() / 1024 / 1024,
        ));
    }

    md::send(bot, chat_id, reply_to, text).await
}
