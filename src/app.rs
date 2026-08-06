use crate::config::Config;
use crate::ui::build_ui;
use gio::prelude::{ApplicationExt, ApplicationExtManual};
use gtk::Application;
use std::rc::Rc;

pub fn run(cfg: Rc<Config>) {
    let app = Application::new(Some("com.scout"), Default::default());

    app.connect_activate(move |app| {
        build_ui(app, cfg.clone()).unwrap_or_else(|e| panic! {"Failed to build UI: {e}"})
    });

    app.run();
}
