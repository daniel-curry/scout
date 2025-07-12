use gtk::gdk::keys::constants::Escape;
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, CssProvider, Entry, EventControllerKey,
    Orientation, StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION,
    Box as GtkBox,
};

fn main() {
    let app = Application::new(Some("com.scout"), Default::default());

    app.connect_activate(|app| {
        let css = "
            entry {
                box-shadow: none;
                border: none;
                outline: none;
                font-family: 'JetBrains Mono';
                font-size: 35px;
                padding: 5px;
                margin: 0px;
            }";

        let provider = CssProvider::new();
        provider.load_from_data(css.as_bytes()).unwrap();
        StyleContext::add_provider_for_screen(
            &gtk::gdk::Screen::default().unwrap(),
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let entry = Entry::new();
        entry.set_hexpand(true);

        let container = GtkBox::new(Orientation::Vertical, 0);
        container.pack_start(&entry, true, true, 0);

        let window = ApplicationWindow::new(app);
        window.set_title("Scout");
        window.set_default_size(550, 40);
        window.set_resizable(false);
        window.set_decorated(false);
        window.add(&container);

        // Exits the application if Escape key is pressed
        let esc_control = EventControllerKey::new(&window);
        let app_clone = app.clone();
        esc_control.connect_key_pressed(move |_, key, _, _| {
            if key == *Escape {
                app_clone.quit();
                return true;
            }
            false
        });

        // Keep controller alive for the window’s lifetime
        unsafe { window.set_data("esc-controller", esc_control); }

        window.show_all();
    });

    app.run();
}

