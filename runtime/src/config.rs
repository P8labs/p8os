use std::env;

#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub root_path: String,
    pub log_path: String,
    pub apps_path: String,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            root_path: "/".into(),
            log_path: "/os/temp/runtime.log".into(),
            apps_path: "/os/apps".into(),
        }
    }
}

impl RuntimeConfig {
    pub fn load() -> Self {
        let mut config = Self::default();

        if let Ok(root) = env::var("ROOT_PATH") {
            let clean_root = root.trim_end_matches('/');
            let log_path = format!("{}/os/temp/runtime.log", clean_root);
            let apps_path = format!("{}/os/apps", clean_root);

            config.log_path = log_path;
            config.root_path = root;
            config.apps_path = apps_path;
        }

        if let Ok(apps_path) = env::var("APPS_PATH") {
            config.apps_path = apps_path;
        }

        config
    }
}
