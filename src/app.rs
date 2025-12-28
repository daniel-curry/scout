use gio::prelude::{ApplicationExt, ApplicationExtManual};
use gtk::Application;
use crate::ui::build_ui;

pub fn run() {
    let app = Application::new(Some("com.scout"), Default::default());

    app.connect_activate(|app| {
        if let Err(e) = build_ui(app) {
            eprintln!("UI error: {e}");
            app.quit();
        }
    });

    app.run();
}