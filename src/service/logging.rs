use std::fs::{self, File, OpenOptions};
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use anyhow::{Context, Result};
use chrono::Local;

pub enum LogMode {
    Foreground,
    Service,
}

struct DailyFile {
    log_dir: PathBuf,
    current_date: String,
    writer: BufWriter<File>,
}

impl DailyFile {
    fn open(log_dir: &Path) -> Result<Self> {
        let today = Local::now().format("%Y-%m-%d").to_string();
        fs::create_dir_all(log_dir).context("cannot create logs directory")?;
        let file_path = log_dir.join(format!("agent_{today}.log"));
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&file_path)
            .with_context(|| format!("cannot open log file: {}", file_path.display()))?;
        Ok(Self {
            log_dir: log_dir.to_path_buf(),
            current_date: today,
            writer: BufWriter::new(file),
        })
    }

    fn rotate_if_needed(&mut self) {
        let today = Local::now().format("%Y-%m-%d").to_string();
        if today == self.current_date {
            return;
        }
        let file_path = self.log_dir.join(format!("agent_{today}.log"));
        match OpenOptions::new()
            .create(true)
            .append(true)
            .open(&file_path)
        {
            Ok(file) => {
                let _ = self.writer.flush();
                self.writer = BufWriter::new(file);
                self.current_date = today;
            }
            Err(e) => {
                eprintln!("Cannot rotate log file: {e}");
            }
        }
    }
}

impl Write for DailyFile {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.rotate_if_needed();
        self.writer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

pub fn init_logger(home_dir: &Path, mode: LogMode) -> Result<()> {
    let log_dir = home_dir.join("logs");
    let file_logger = DailyFile::open(&log_dir)?;
    let file_mutex = Mutex::new(file_logger);

    let file_dispatch = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] [{:<5}] [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(fern::Output::call(move |record| {
            if let Ok(mut guard) = file_mutex.lock() {
                let _ = writeln!(guard, "{}", record.args());
                let _ = guard.flush();
            }
        }));

    match mode {
        LogMode::Foreground => {
            let stderr_dispatch = fern::Dispatch::new()
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "[{}] [{:<5}] [{}] {}",
                        Local::now().format("%Y-%m-%d %H:%M:%S"),
                        record.level(),
                        record.target(),
                        message
                    ))
                })
                .level(log::LevelFilter::Debug)
                .chain(io::stderr());

            fern::Dispatch::new()
                .chain(file_dispatch)
                .chain(stderr_dispatch)
                .apply()
                .context("cannot apply logger")?;
        }
        LogMode::Service => {
            file_dispatch.apply().context("cannot apply logger")?;
        }
    }

    Ok(())
}
