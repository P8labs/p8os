use std::path::PathBuf;

use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::prelude::*;
use webkit6::prelude::WebViewExt;

use crate::core::error::RuntimeResult;
use crate::window::webview::WebViewManager;

pub struct ShellHost;

impl ShellHost {
    pub fn launch(application: &Application, shell_path: &PathBuf) -> RuntimeResult<()> {
        let window = ApplicationWindow::builder()
            .application(application)
            .title("P8 Operating System")
            .build();

        window.fullscreen();

        let shell_path = shell_path.canonicalize()?;
        let webview = WebViewManager::create(&window)?;
        webview.load_uri(&format!("file://{}", shell_path.to_string_lossy()));
        window.set_child(Some(&webview));
        window.present();

        window.connect_close_request(|_| {
            eprintln!("Shell window close blocked");
            gtk4::glib::Propagation::Stop
        });
        Ok(())
    }
}
