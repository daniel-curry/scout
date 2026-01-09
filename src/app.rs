use std::rc::Rc;
use gio::prelude::{ApplicationExt, ApplicationExtManual};
use gtk::Application;
use crate::config::Config;
use crate::ui::build_ui;

pub fn run(cfg: Rc<Config>) {
    let app = Application::new(Some("com.scout"), Default::default());

    let cfg_clone = cfg.clone();
    app.connect_activate(move|app| {
        let cfg_inner = cfg_clone.clone();
        if let Err(e) = build_ui(app, cfg_inner) {
            eprintln!("UI error: {e}");
            app.quit();
        }
    });

    app.run();
}