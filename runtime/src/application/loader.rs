use std::fs;

use crate::{
    application::{app::App, permissions::verify_permissions},
    config::RuntimeConfig,
    core::error::RuntimeResult,
};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Manifest {
    id: String,
    permissions: Vec<String>,
    icon: Option<String>,
    entry: String,
}

const SYSTEM_APPS: &[&str] = &["os.shell.app", "os.settings.app", "os.files.app"];

pub struct ApplicationLoader {
    apps: Vec<App>,
}

fn discover_apps(apps_path: &str) -> RuntimeResult<Vec<App>> {
    let mut apps = Vec::new();
    let entries = fs::read_dir(apps_path)?;
    for entry in entries {
        let entry = entry?;

        if !entry.file_type()?.is_dir() {
            continue;
        }

        let app_dir = entry.path();

        let manifest_path = app_dir.join("manifest.json");

        if !manifest_path.exists() {
            continue;
        }

        let manifest_raw = fs::read_to_string(&manifest_path)?;

        let manifest: Manifest = serde_json::from_str(&manifest_raw)?;

        verify_permissions(&manifest.permissions)?;

        let is_sys = SYSTEM_APPS.iter().any(|app_id| *app_id == manifest.id);

        apps.push(App {
            id: manifest.id,
            path: app_dir.clone(),
            entry: app_dir.join(&manifest.entry),
            permissions: manifest.permissions,
            icon: manifest.icon,
            is_sys,
        });
    }
    Ok(apps)
}

impl ApplicationLoader {
    pub fn new(config: &RuntimeConfig) -> RuntimeResult<Self> {
        let apps = discover_apps(&config.apps_path)?;

        Ok(Self { apps })
    }

    pub fn find_app_by_id(&self, id: &str) -> Option<&App> {
        self.apps.iter().find(|a| a.id == id)
    }
}
