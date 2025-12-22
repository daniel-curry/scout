use std::cell::RefCell;
use std::rc::Rc;

use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use gdk::glib::Propagation;
use gdk::prelude::*;
use gio::AppInfo;
use gio::prelude::AppInfoExt;
use glib::SpawnFlags;

use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box as GtkBox, Entry, Label, ListBox, ListBoxRow, Orientation,
};

use gdk::keys::constants as key;

fn main() {
    let app = Application::new(Some("com.scout"), Default::default());

    app.connect_activate(|app| {
        if let Err(e) = build_ui(app) {
            eprintln!("UI error: {e}");
            app.quit();
        }
    });

    app.run();
}

fn build_ui(app: &Application) -> Result<(), String> {
    // Data
    let all_apps = Rc::new(get_apps());
    let current_results: Rc<RefCell<Vec<AppInfo>>> = Rc::new(RefCell::new(Vec::new()));

    // Window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Scout")
        .default_width(600)
        .default_height(260)
        .resizable(false)
        .decorated(false)
        .build();

    window.set_position(gtk::WindowPosition::Center);
    window.set_keep_above(true);

    // Layout
    let vbox = GtkBox::new(Orientation::Vertical, 8);
    vbox.set_margin_top(16);
    vbox.set_margin_bottom(16);
    vbox.set_margin_start(16);
    vbox.set_margin_end(16);

    let entry = Entry::new();
    entry.set_placeholder_text(Some("Type to search…"));
    vbox.pack_start(&entry, false, false, 0);

    let list = ListBox::new();
    list.set_activate_on_single_click(true);
    list.set_selection_mode(gtk::SelectionMode::Single);
    vbox.pack_start(&list, true, true, 0);

    window.add(&vbox);

    // Render helper: compute top matches, rebuild rows, select first.
    let render_results = {
        let list = list.clone();
        let all_apps = all_apps.clone();
        let current_results = current_results.clone();
        move |query: &str| {
            // Clear rows
            list.foreach(|child| list.remove(child));

            // Compute matches
            let matches = top_matches(&all_apps, query, 5);

            // Update "model" backing the list
            *current_results.borrow_mut() = matches.clone();

            // Add rows
            for app in matches {
                list.add(&render_row(&app));
            }

            list.show_all();
            if let Some(row) = list.row_at_index(0) {
                list.select_row(Some(&row));
            }
        }
    };

    // Initial fill (top 5 apps)
    render_results("");

    // Update list on typing
    entry.connect_changed({
        let render_results = render_results.clone();
        move |e| {
            let text = e.text().to_string();
            render_results(&text);
        }
    });

    // Launch on row activation (double click or Enter activation)
    list.connect_row_activated({
        let current_results = current_results.clone();
        let window_clone = window.clone();
        let app_clone = app.clone();
        move |_, row| {
            let idx = row.index() as usize;
            let maybe_app = current_results.borrow().get(idx).cloned();
            if let Some(appinfo) = maybe_app {
                // Hide window immediately for better UX
                window_clone.hide();

                if let Err(err) = launch_app(&appinfo) {
                    eprintln!("Launch failed: {err}");
                } else {
                    // Give the shell time to fork the process before we exit
                    // This ensures the launched app is fully detached
                    let app_ref = app_clone.clone();
                    app_ref.quit();
                }
            }
        }
    });

    // Key handling: Up/Down, Enter, Escape
    window.connect_key_press_event({
        let list = list.clone();
        let app_clone = app.clone();
        move |_, ev| {
            let keyval = ev.keyval();

            if keyval == key::Escape {
                app_clone.quit();
                return Propagation::Stop;
            }

            if keyval == key::Up {
                if let Some(sel) = list.selected_row() {
                    let idx = sel.index().max(1) - 1;
                    if let Some(row) = list.row_at_index(idx) {
                        list.select_row(Some(&row));
                    }
                } else if let Some(row) = list.row_at_index(0) {
                    list.select_row(Some(&row));
                }
                return Propagation::Stop;
            }

            if keyval == key::Down {
                if let Some(sel) = list.selected_row() {
                    let idx = sel.index() + 1;
                    if let Some(row) = list.row_at_index(idx) {
                        list.select_row(Some(&row));
                    }
                } else if let Some(row) = list.row_at_index(0) {
                    list.select_row(Some(&row));
                }
                return Propagation::Stop;
            }

            if (keyval == key::Return || keyval == key::KP_Enter)
                && list
                    .selected_row()
                    .map(|sel| {
                        sel.activate();
                        true
                    })
                    .unwrap_or(false)
            {
                return Propagation::Stop;
            }

            Propagation::Proceed
        }
    });

    window.show_all();
    entry.grab_focus();
    Ok(())
}

fn render_row(app: &AppInfo) -> ListBoxRow {
    let row = ListBoxRow::new();
    let label = Label::new(Some(&app.name()));
    label.set_xalign(0.0);
    row.add(&label);
    row
}

fn top_matches(apps: &[AppInfo], query: &str, k: usize) -> Vec<AppInfo> {
    let q = query.trim();
    if q.is_empty() {
        return apps.iter().take(k).cloned().collect();
    }

    let matcher = SkimMatcherV2::default();
    let mut scored: Vec<(i64, &AppInfo)> = apps
        .iter()
        .filter_map(|app| matcher.fuzzy_match(&app.name(), q).map(|s| (s, app)))
        .collect();

    scored.sort_by(|a, b| b.0.cmp(&a.0));
    scored.into_iter().take(k).map(|(_, a)| a.clone()).collect()
}

fn get_apps() -> Vec<AppInfo> {
    AppInfo::all()
        .into_iter()
        .filter(|a| a.should_show())
        .collect()
}

fn needs_terminal(app: &AppInfo) -> bool {
    if let Some(dai) = app.downcast_ref::<gio::DesktopAppInfo>() {
        return dai.boolean("Terminal");
    }
    false
}

pub fn launch_gui_app(app: &gio::AppInfo) -> Result<(), String> {
    let ctx = gio::AppLaunchContext::new();

    // Prefer DesktopAppInfo so we can inject a child-setup hook (setsid).
    if let Some(dai) = app.dynamic_cast_ref::<gio::DesktopAppInfo>() {
        // No URIs/files to pass
        let uris: [&str; 0] = [];

        let spawn_flags =
            SpawnFlags::SEARCH_PATH
            | SpawnFlags::STDOUT_TO_DEV_NULL
            | SpawnFlags::STDERR_TO_DEV_NULL;

        // Called after fork() but before exec() in the child.
        let user_setup: Option<Box<dyn FnOnce()>> = Some(Box::new(|| {
            #[cfg(unix)]
            unsafe {
                let _ = libc::setsid();
            }
        }));

        dai.launch_uris_as_manager(&uris, Some(&ctx), spawn_flags, user_setup, None)
            .map_err(|e| format!("Failed to launch app '{}': {}", app.name(), e))?;

        return Ok(());
    }
    Ok(())
}
