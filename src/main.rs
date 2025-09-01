use gtk::prelude::*;
use gtk::gdk::keys::constants::Escape;
use gtk::{
    Application, ApplicationWindow, CssProvider, Entry, EventControllerKey,
    Orientation, StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION, Box as GtkBox};

use gio::prelude::AppInfoExt;
use gio::{AppInfo};

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

fn main() {
    let gtk_app = Application::new(Some("com.scout"), Default::default());
    gtk_app.connect_activate(move|gtk_app| {
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

        build_ui(gtk_app);
        let window = gtk_app.active_window().unwrap();
        let entry = window.child().unwrap().downcast::<GtkBox>().unwrap()
            .children()[0]
            .clone()
            .downcast::<Entry>()
            .unwrap();

        // Exits the application if Escape key is pressed
        let esc_control = EventControllerKey::new(&window);
        let app_clone = gtk_app.clone();
        esc_control.connect_key_pressed(move |_, key, _, _| {
            if key == *Escape {
                app_clone.quit();
                return true;
            }
            false
        });

        // Keep controller alive for the window’s lifetime
        unsafe { window.set_data("esc-controller", esc_control); }

        // Fuzzy matching logic
        let installed_apps = get_apps();
        let apps = installed_apps.clone();
        let matcher = SkimMatcherV2::default();
        entry.connect_changed(move |e| {
            let text = e.text().to_string();
            if text.is_empty() {
                println!("No input");
                return;
            }
            let mut results: Vec<(&AppInfo, i64)> = apps.iter()
                .filter_map(|app| {
                    matcher.fuzzy_match(&app.name(), &text)
                        .map(|score| (app, score))
                })
                .collect();
            results.sort_by(|a, b| b.1.cmp(&a.1));
            println!("Results for '{}':", text);
            for (app, score) in results.iter().take(5) {
                println!("{} (score: {})", app.name(), score);
            }
        });
    });
    gtk_app.run();
}

pub fn get_apps() -> Vec<AppInfo> {
        AppInfo::all()
            .into_iter()
            .filter(|a| a.should_show())         
            .collect()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Scout")
        .default_width(550)
        .default_height(40)
        .resizable(false)
        .decorated(false)
        .build();

    let entry = Entry::new();
    entry.set_hexpand(true);

    let container = GtkBox::new(Orientation::Vertical, 0);
    container.pack_start(&entry, true, true, 0);
    window.set_child(Some(&container));
    window.show_all();
}
