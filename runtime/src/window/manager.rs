use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::prelude::*;

use crate::core::error::RuntimeResult;

pub struct WindowManager {
    application: Application,
}

impl WindowManager {
    pub fn new(application: Application) -> RuntimeResult<Self> {
        Ok(Self { application })
    }

    pub fn create_main_window(&self) -> RuntimeResult<ApplicationWindow> {
        let window = ApplicationWindow::builder()
            .application(&self.application)
            .title("P8OS")
            .default_width(1280)
            .default_height(720)
            .build();

        window.fullscreen();

        Ok(window)
    }
}
