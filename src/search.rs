use std::rc::Rc;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use gio::AppInfo;
use gio::prelude::AppInfoExt;
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

pub fn top_matches(entries: &Rc<Vec<Entry>>, query: &str, k: usize) -> Vec<Entry> {
    let q = query.trim();
    if q.is_empty() {
        return entries.iter().take(k).cloned().collect();
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
    scored.into_iter().take(k).map(|(_, e)| e.clone()).collect()
}