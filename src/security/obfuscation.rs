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
