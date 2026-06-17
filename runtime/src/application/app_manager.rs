use crate::{application::app::App, core::error::RuntimeResult};

pub struct AppManager;

impl AppManager {
    pub fn new() -> RuntimeResult<Self> {
        Ok(Self)
    }

    pub fn launch(&self, app: &App) -> RuntimeResult<()> {
        println!("Launching app: {}", app.id);
        Ok(())
    }

    pub fn launch_url(&self, url: &str) -> RuntimeResult<()> {
        println!("Opening URL: {}", url);
        Ok(())
    }
}
