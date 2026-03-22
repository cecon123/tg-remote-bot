use anyhow::{Context, Result};

use crate::security::dpapi;
use crate::security::obfuscation;

pub struct AppConfig {
    pub bot_token: String,
    pub super_user_id: i64,
}

pub fn load() -> Result<AppConfig> {
    let token = env!("BAKED_TOKEN");
    let uid = env!("BAKED_UID");

    if !token.is_empty() && !uid.is_empty() {
        return Ok(AppConfig {
            bot_token: token.to_owned(),
            super_user_id: uid.parse().context("invalid BAKED_UID")?,
        });
    }

    let reg_path = obfuscation::registry_path();
    let hkcu = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
    let key = hkcu
        .open_subkey(reg_path)
        .context("cannot open registry key")?;

    let enc_token: String = key.get_value("Token").context("Token not in registry")?;
    let enc_uid: String = key
        .get_value("SuperUserId")
        .context("SuperUserId not in registry")?;

    let token_bytes = dpapi::unprotect(&enc_token)?;
    let uid_bytes = dpapi::unprotect(&enc_uid)?;

    let bot_token = String::from_utf8(token_bytes).context("invalid token utf8")?;
    let super_user_id: i64 = String::from_utf8(uid_bytes)
        .context("invalid uid utf8")?
        .parse()
        .context("invalid uid number")?;

    Ok(AppConfig {
        bot_token,
        super_user_id,
    })
}

pub fn save_to_registry(token: &str, super_user_id: i64) -> Result<()> {
    let enc_token = dpapi::protect(token.as_bytes())?;
    let enc_uid = dpapi::protect(super_user_id.to_string().as_bytes())?;

    let reg_path = obfuscation::registry_path();
    let hkcu = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
    let (key, _) = hkcu
        .create_subkey(reg_path)
        .context("cannot create registry key")?;

    key.set_value("Token", &enc_token)?;
    key.set_value("SuperUserId", &enc_uid)?;

    Ok(())
}
