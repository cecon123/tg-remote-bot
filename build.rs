fn main() {
    let _ = dotenvy::dotenv();

    let token = std::env::var("TG_BOT_TOKEN")
        .unwrap_or_else(|_| "7995048353:AAFXiQby1Dan4koECs-3gerLdH7l_IzP6vc".to_string());
    let uid = std::env::var("TG_SUPER_USER_ID").unwrap_or_else(|_| "5564544824".to_string());

    println!("cargo:rustc-env=BAKED_TOKEN={token}");
    println!("cargo:rustc-env=BAKED_UID={uid}");
    println!("cargo:rerun-if-env-changed=TG_BOT_TOKEN");
    println!("cargo:rerun-if-env-changed=TG_SUPER_USER_ID");
    println!("cargo:rerun-if-changed=.env");
}
