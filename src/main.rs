use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

const APP_ID: &str = "org.pixelpotato.nugget";

fn main() {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Hello, World!")
            .build();

        window.show();
    });

    app.run();
}
