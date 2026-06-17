use gtk4::{
    Application,
    gio::prelude::{ApplicationExt, ApplicationExtManual},
};
use runtime::{
    application::loader::ApplicationLoader,
    config::RuntimeConfig,
    core::error::{RuntimeError, RuntimeResult},
    emergency::emergency_mode,
    window::shell_host::ShellHost,
};
use std::{path::Path, process};

pub struct Runtime {
    pub config: RuntimeConfig,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            config: RuntimeConfig::load(),
        }
    }
}

fn main() {
    let runtime = Runtime::new();

    let app = Application::builder()
        .application_id("com.p8os.runtime")
        .build();

    app.connect_activate(move |app| {
        if let Err(err) = boot(&runtime.config, app) {
            emergency_mode(&runtime.config, &err);
        }
    });

    app.run();
}

fn boot(config: &RuntimeConfig, application: &Application) -> RuntimeResult<()> {
    let pid = process::id();

    println!("booting...");

    if pid == 1 {
        println!("Running as PID 1");
        // TODO: mount filesystems
        // TODO: initialize runtime services
    } else {
        println!("Running in development mode");
    }

    let loader = ApplicationLoader::new(config)?;

    let shell = loader
        .find_app_by_id("os.shell.app")
        .ok_or_else(|| RuntimeError::Boot("System shell (os.shell.app) not found".into()))?;

    if !Path::new(&shell.entry).exists() {
        return Err(RuntimeError::Boot(format!(
            "Shell entrypoint not found: {}",
            shell.entry.to_string_lossy()
        )));
    }

    ShellHost::launch(application, &shell.entry)?;

    Ok(())
}
