use std::rc::Rc;
use std::sync::Arc;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use gio::AppInfo;
use gio::prelude::AppInfoExt;
use crate::config::Config;
use crate::entry::{Entry, SystemAction};

pub fn get_entries() -> Vec<Entry> {
    let mut entries: Vec<Entry> = AppInfo::all()
        .into_iter()
        .filter(|a| a.should_show())
        .map(Entry::from_app)
        .collect();

    entries.push(Entry::system_action(SystemAction::Shutdown));
    entries.push(Entry::system_action(SystemAction::Restart));
    entries.push(Entry::system_action(SystemAction::Sleep));
    entries.push(Entry::system_action(SystemAction::Hibernate));

    entries
}

pub fn top_matches(entries: &Rc<Vec<Entry>>, query: &str, cfg: Arc<Config>) -> Vec<Entry> {
    let q = query.trim();
    if q.is_empty() {
        return entries.iter().take(cfg.max_results).cloned().collect();
    }

    let matcher = SkimMatcherV2::default();
    let mut scored: Vec<(i64, &Entry)> = entries
        .iter()
        .filter_map(|entry| {
            matcher
                .fuzzy_match(&entry.title, q)
                .map(|score| (score, entry))
        })
        .collect();

    scored.sort_by(|a, b| b.0.cmp(&a.0));
    scored.into_iter().take(cfg.max_results).map(|(_, e)| e.clone()).collect()
}