use crate::core::error::{RuntimeError, RuntimeResult};

const KNOWN_PERMISSIONS: &[&str] = &["storage.read", "storage.write", "window.open"];

pub fn verify_permissions(permissions: &[String]) -> RuntimeResult<()> {
    for permission in permissions {
        if !KNOWN_PERMISSIONS.contains(&permission.as_str()) {
            return Err(RuntimeError::App(format!(
                "Unknown permission: {}",
                permission
            )));
        }
    }

    Ok(())
}
