use crate::config::{Config, Theme};
use crate::entry::{ActivateAction, Launch};
use crate::search::{get_entries, top_matches};

use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use gdk::{glib::Propagation, keys::constants as key};
use gtk::Orientation::Vertical;
use gtk::{
    Application, ApplicationWindow, Box as GtkBox, CssProvider, Entry as GtkEntry, Label, ListBox,
    ListBoxRow, Orientation, SelectionMode, WindowPosition, prelude::*,
};
use std::cell::Cell;
use std::{cell::RefCell, rc::Rc};

pub fn build_ui(app: &Application, cfg: Rc<Config>) -> Result<(), String> {
    let current_results: Rc<RefCell<Vec<Rc<dyn Launch>>>> = Rc::new(RefCell::new(Vec::new()));
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Scout")
        .default_width(cfg.window_width)
        .default_height(cfg.window_height)
        .resizable(false)
        .decorated(true)
        .window_position(WindowPosition::Center)
        .build();

    let entry = GtkEntry::new();
    entry.set_text("Type to search...");

    let vbox = GtkBox::builder()
        .orientation(Vertical)
        .spacing(8)
        .margin(16)
        .build();
    vbox.pack_start(&entry, false, false, 0);
    window.add(&vbox);
    install_global_css(&css_from_config(&cfg.theme));

    let list = ListBox::builder()
        .activate_on_single_click(true)
        .selection_mode(SelectionMode::Single)
        .build();

    vbox.pack_start(&list, true, true, 0);

    let binding = top_matches(app, "");
    let matches = binding.iter().take(cfg.max_results);

    for entry in matches {
        list.add(&render_row(entry, &cfg));
    }
    list.show_all();

    let hint_cleared: Rc<Cell<bool>> = Rc::new(Cell::new(false));

    let app_list = get_entries(app);
    let max_results = cfg.max_results;

    // Update list on typing
    entry.connect_changed({
        let list = list.clone();
        let cfg = Rc::clone(&cfg);
        let current_results = current_results.clone();
        move |entry| {
            let matches = matcher(app_list.to_vec(), &entry.text(), max_results);
            list.foreach(|child| list.remove(child));
            for row in &matches {
                list.add(&render_row(row, &cfg));
            }
            list.show_all();
            if let Some(row) = list.row_at_index(0) {
                list.select_row(Some(&row));
            }

            *current_results.borrow_mut() = matches.clone();
        }
    });

    // Clear the hint text when user starts typing
    entry.connect_key_press_event({
        let hint_cleared = hint_cleared.clone();
        move |e, ev| {
            if !hint_cleared.get() {
                let keyval = ev.keyval();
                // Only clear on printable characters, not navigation keys
                if let Some(c) = keyval.to_unicode()
                    && !c.is_control()
                {
                    e.set_text("");
                    hint_cleared.set(true);
                }
            }
            Propagation::Proceed
        }
    });

    // Launch on row activation (double click or Enter activation)
    list.connect_row_activated({
        // let current_results =  list.clone();
        let window_clone = window.clone();
        let app_clone = app.clone();
        let current_results = current_results.clone();
        move |_, row| {
            let idx = row.index() as usize;
            let maybe_entry = current_results.borrow().get(idx).cloned();
            if let Some(entry) = maybe_entry {
                // Hide window immediately for better UX
                window_clone.hide();

                match entry.activate(&cfg) {
                    ActivateAction::Dismiss => app_clone.quit(),
                    ActivateAction::KeepOpen => (),
                }
            }
        }
    });

    // Key handling: Up/Down, Enter, Escape
    window.connect_key_press_event({
        let list = list.clone();
        let app_clone = app.clone();
        move |_, ev| match ev.keyval() {
            key::Escape => {
                app_clone.quit();
                Propagation::Stop
            }
            key::Up => {
                if let Some(sel) = list.selected_row() {
                    let idx = sel.index().max(1) - 1;
                    if let Some(row) = list.row_at_index(idx) {
                        list.select_row(Some(&row));
                    }
                } else if let Some(row) = list.row_at_index(0) {
                    list.select_row(Some(&row));
                }
                Propagation::Stop
            }
            key::Down => {
                if let Some(sel) = list.selected_row() {
                    let idx = sel.index() + 1;
                    if let Some(row) = list.row_at_index(idx) {
                        list.select_row(Some(&row));
                    }
                } else if let Some(row) = list.row_at_index(0) {
                    list.select_row(Some(&row));
                }
                Propagation::Stop
            }
            key::Return | key::KP_Enter => {
                if list
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
            _ => Propagation::Proceed,
        }
    });

    window.show_all();
    entry.grab_focus();
    entry.select_region(0, 0);

    Ok(())
}

fn matcher(app_list: Vec<Rc<dyn Launch>>, query: &str, max_results: usize) -> Vec<Rc<dyn Launch>> {
    let mut scored: Vec<(i64, Rc<dyn Launch>)> = app_list
        .iter()
        .filter_map(|entry| {
            SkimMatcherV2::default()
                .fuzzy_match(&entry.title(), query)
                .map(|score| (score, entry.clone()))
        })
        .collect::<Vec<_>>();

    scored.sort_by_key(|b| std::cmp::Reverse(b.0));

    scored
        .iter()
        .map(|v| v.1.clone())
        .take(max_results)
        .collect()
}

pub fn render_row(entry: &Rc<impl Launch + ?Sized>, cfg: &Config) -> ListBoxRow {
    let row = ListBoxRow::new();
    let hbox = GtkBox::new(Orientation::Horizontal, 8);

    if cfg.show_icons {
        let icon = entry.icon(cfg);
        hbox.pack_start(&icon, false, false, 0);
    }

    let label = Label::new(Some(&entry.title()));
    label.set_xalign(0.0);
    hbox.pack_start(&label, true, true, 0);

    row.add(&hbox);
    row
}

pub fn install_global_css(css: &str) {
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
