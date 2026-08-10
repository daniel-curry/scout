use crate::entry::{AppEntry, Launch, SystemAction, SystemPowerEntry};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use gio::AppInfo;
use gio::prelude::AppInfoExt;
use std::rc::Rc;

pub fn get_entries(application: &gtk::Application) -> Vec<Rc<dyn Launch>> {
    let mut entries: Vec<Rc<dyn Launch>> = AppInfo::all()
        .into_iter()
        .filter(|a| a.should_show())
        .map(|app_info| {
            Rc::new(AppEntry {
                app: application.clone(),
                info: app_info,
            }) as Rc<dyn Launch>
        })
        .collect();

    entries.push(Rc::new(SystemPowerEntry {
        action: SystemAction::Shutdown,
    }) as Rc<dyn Launch>);
    entries.push(Rc::new(SystemPowerEntry {
        action: SystemAction::Restart,
    }) as Rc<dyn Launch>);
    entries.push(Rc::new(SystemPowerEntry {
        action: SystemAction::Sleep,
    }) as Rc<dyn Launch>);
    entries.push(Rc::new(SystemPowerEntry {
        action: SystemAction::Hibernate,
    }) as Rc<dyn Launch>);

    entries
}

pub fn top_matches(app: &gtk::Application, query: &str) -> Vec<Rc<dyn Launch>> {
    if query.is_empty() {
        return get_entries(app);
    }

    let scored: Vec<(i64, Rc<dyn Launch>)> = get_entries(app)
        .iter()
        .filter_map(|entry| {
            SkimMatcherV2::default()
                .fuzzy_match(&entry.title(), query)
                .map(|score| (score, entry.clone()))
        })
        .collect::<Vec<_>>();

    scored.iter().map(|v| v.1.clone()).collect()
}
