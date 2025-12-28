use gtk::{Label, ListBoxRow};
use gtk::prelude::{ContainerExt, LabelExt};
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box as GtkBox, Entry as GtkEntry, ListBox, Orientation,
};

use std::cell::RefCell;
use std::rc::Rc;

use gdk::glib::Propagation;
use gdk::keys::constants as key;

use crate::entry::{Entry, EntryKind};
use crate::launcher::{launch_gui_app, launch_terminal_application, needs_terminal};
use crate::search::{get_entries, top_matches};
use crate::config::{TERMINAL_EMULATOR, WINDOW_HEIGHT, WINDOW_WIDTH};

pub fn build_ui(app: &Application) -> Result<(), String> {
    // Data
    let all_apps = Rc::new(get_entries());
    let current_results: Rc<RefCell<Vec<Entry>>> = Rc::new(RefCell::new(Vec::new()));

    // Window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Scout")
        .default_width(WINDOW_WIDTH)
        .default_height(WINDOW_HEIGHT)
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

    let entry = GtkEntry::new();
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
            let matches = top_matches(&all_apps, query);

            // Update "model" backing the list
            *current_results.borrow_mut() = matches.clone();

            // Add rows
            for entry in matches {
                list.add(&render_row(&entry));
            }

            list.show_all();
            if let Some(row) = list.row_at_index(0) {
                list.select_row(Some(&row));
            }
        }
    };

    // Initial fill (top k apps)
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
            let maybe_entry = current_results.borrow().get(idx).cloned();
            if let Some(entry) = maybe_entry {
                // Hide window immediately for better UX
                window_clone.hide();

                match &entry.kind {
                    EntryKind::App(appinfo) => {
                        if needs_terminal(appinfo) {
                            let exec_path = appinfo.executable();
                            let exec = exec_path.to_string_lossy().into_owned();
                            let term = TERMINAL_EMULATOR.to_string();
                            launch_terminal_application(&[exec], &[term])
                                .map_err(|e| format!("Failed to launch terminal app: {}", e))
                                .unwrap_or_else(|err| eprintln!("Launch failed: {err}"));
                            app_clone.quit();
                            return;
                        }

                        if let Err(err) = launch_gui_app(appinfo) {
                            eprintln!("Launch failed: {err}");
                        } else {
                            app_clone.quit();
                        }
                    }
                    EntryKind::Action(action) => {
                        if let Err(err) = crate::launcher::launch_system_action(action) {
                            eprintln!("Action failed: {err}");
                        } else {
                            app_clone.quit();
                        }
                    }
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

pub fn render_row(entry: &Entry) -> ListBoxRow {
    let row = ListBoxRow::new();
    let label = Label::new(Some(&entry.title));
    label.set_xalign(0.0);
    row.add(&label);
    row
}