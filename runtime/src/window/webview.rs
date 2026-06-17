use gtk4::ApplicationWindow;
use webkit6::WebView;

use crate::core::error::RuntimeResult;

pub struct WebViewManager;

impl WebViewManager {
    pub fn create(_window: &ApplicationWindow) -> RuntimeResult<WebView> {
        let webview = WebView::new();

        Ok(webview)
    }
}
