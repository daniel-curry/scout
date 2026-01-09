use gtk::prelude::*;
use gtk::prelude::{ContainerExt, LabelExt};
use gtk::{Application, ApplicationWindow, Box as GtkBox, CssProvider, Entry as GtkEntry, Image, ListBox, Orientation};
use gtk::{Label, ListBoxRow};

use std::cell::RefCell;
use std::rc::Rc;
use gdk::glib::Propagation;
use gdk::keys::constants as key;
use crate::config::{Config, Theme};
use crate::entry::{Entry, EntryKind};
use crate::icon::{create_app_icon_widget, create_generic_icon_widget};
use crate::launcher::{launch_gui_app, launch_terminal_application, needs_terminal};
use crate::search::{get_entries, top_matches};

pub fn build_ui(app: &Application, cfg: Rc<Config>) -> Result<(), String> {
    // Data
    let all_apps = Rc::new(get_entries());
    let current_results: Rc<RefCell<Vec<Entry>>> = Rc::new(RefCell::new(Vec::new()));

    // Window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Scout")
        .default_width(cfg.window_width)
        .default_height(cfg.window_height)
        .resizable(false)
        .decorated(true)
        .build();

    window.set_position(gtk::WindowPosition::Center);
    window.set_keep_above(true);

    let css = css_from_config(&cfg.theme);
    install_global_css(&css);


    // Layout
    let vbox = GtkBox::new(Orientation::Vertical, 8);
    vbox.set_margin_top(16);
    vbox.set_margin_bottom(16);
    vbox.set_margin_start(16);
    vbox.set_margin_end(16);

    let entry = GtkEntry::new();
    entry.set_text("Type to search…");
    vbox.pack_start(&entry, false, false, 0);

    let list = ListBox::new();
    list.set_activate_on_single_click(true);
    list.set_selection_mode(gtk::SelectionMode::Single);
    vbox.pack_start(&list, true, true, 0);

    window.add(&vbox);

    // Track if this is the first change (to clear hint text)
    let hint_cleared: Rc<RefCell<bool>> = Rc::new(RefCell::new(false));

    // Render helper: compute top matches, rebuild rows, select first.
    let render_results = {
        let list = list.clone();
        let all_apps = all_apps.clone();
        let cfg = cfg.clone();
        let current_results = current_results.clone();
        move |query: &str| {
            // Clear rows
            list.foreach(|child| list.remove(child));

            // Compute matches
            let match_cfg = cfg.clone();
            let matches = top_matches(&all_apps, query, match_cfg);

            // Update "model" backing the list
            *current_results.borrow_mut() = matches.clone();

            //Create new config

            // Add rows
            for entry in matches {
                let inner_cfg = cfg.clone();
                list.add(&render_row(&entry, inner_cfg));
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
        let hint_cleared = hint_cleared.clone();
        move |e| {
            let text = e.text().to_string();

            // If hint hasn't been cleared yet and text is the hint, skip
            if !*hint_cleared.borrow() {
                if text == "Type to search…" {
                    return;
                }
                *hint_cleared.borrow_mut() = true;
            }

            render_results(&text);
        }
    });

    // Clear the hint text when user starts typing
    entry.connect_key_press_event({
        let hint_cleared = hint_cleared.clone();
        move |e, ev| {
            if !*hint_cleared.borrow() {
                let keyval = ev.keyval();
                // Only clear on printable characters, not navigation keys
                if let Some(c) = keyval.to_unicode() && !c.is_control() {
                        e.set_text("");
                        *hint_cleared.borrow_mut() = true;
                }
            }
            Propagation::Proceed
        }
    });

    // Launch on row activation (double click or Enter activation)
    list.connect_row_activated({
        let current_results = current_results.clone();
        let window_clone = window.clone();
        let app_clone = app.clone();
        let cfg = cfg.clone();
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
                            let term = cfg.terminal_emulator.to_string();
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

                    EntryKind::Result(_math_entry) => {
                        // Do nothing; we just show the result
                        app_clone.quit();
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
    // Deselect the hint text so it's visible but not selected
    entry.select_region(0, 0);
    Ok(())
}

pub fn render_icon(entry: &Entry, cfg: Rc<Config>) -> Image {
    match &entry.kind {
        EntryKind::App(appinfo) => create_app_icon_widget(appinfo, cfg),
        EntryKind::Action(_) => create_generic_icon_widget("system-shutdown", cfg),
        EntryKind::Result(_) => create_generic_icon_widget("accessories-calculator", cfg),
    }
}

pub fn render_row(entry: &Entry, cfg: Rc<Config>) -> ListBoxRow {
    let row = ListBoxRow::new();
    let hbox = GtkBox::new(Orientation::Horizontal, 8);

    if cfg.show_icons {
        let icon = render_icon(entry, cfg);
        hbox.pack_start(&icon, false, false, 0);
    }

    let label = Label::new(Some(&entry.title));
    label.set_xalign(0.0);
    hbox.pack_start(&label, true, true, 0);

    row.add(&hbox);
    row
}

pub fn install_global_css(css: &str) {
    // Create provider + load CSS
    let provider = CssProvider::new();
    provider
        .load_from_data(css.as_bytes())
        .expect("Failed to load CSS");

    // Apply to the whole screen (global)
    if let Some(screen) = gdk::Screen::default() {
        gtk::StyleContext::add_provider_for_screen(
            &screen,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    } else {
        eprintln!("No default GDK screen available");
    }
}

pub fn css_from_config(theme: &Theme) -> String {
    format!(
        r#"
        window {{
            background-color: {bg_color};
        }}

        entry {{
            font-family: "{font_family}";
            font-size: {font_size}pt;
            color: {font_color};
            background-color: {bg_color};
            min-height: {entry_min_height}px;
            border: 1px solid {entry_border_color};
            border-radius: {entry_border_radius}px;
        }}

        entry:focus {{
            outline: none;
            box-shadow: none;
            border-color: {entry_border_color};
        }}

        label {{
            font-family: "{font_family}";
            font-size: {font_size}pt;
            color: {font_color};
        }}

        list {{
            background-color: {bg_color};
        }}

        row {{
            background-color: {bg_color};
        }}

        row:selected {{
            background-color: {selection_color};
        }}
        "#,
        font_family = theme.font_family,
        font_size = theme.font_size,
        bg_color = theme.bg_color,
        font_color = theme.font_color,
        selection_color = theme.selection_color,
        entry_min_height = theme.entry_min_height,
        entry_border_color = theme.entry_border_color,
        entry_border_radius = theme.entry_border_radius,
    )
}
