use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};

use webkit6::WebView;
use webkit6::prelude::WebViewExt;

fn main() {
    let app = Application::builder()
        .application_id("in.p8labs.os.runtime")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("P8OS Runtime")
        .default_width(1280)
        .default_height(720)
        .build();

    let webview = WebView::new();

    webview.load_html(
        r#"
        <!DOCTYPE html>
        <html>
        <body>
            <h1>P8OS Runtime</h1>
            <p>Hello from WebKitGTK.</p>
        </body>
        </html>
        "#,
        None,
    );

    window.set_child(Some(&webview));

    window.present();
}
