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
    let mut scored: Vec<(i64, Entry)> = entries
        .iter()
        .filter_map(|entry| {
            matcher
                .fuzzy_match(&entry.title, q)
                .map(|score| (score, entry.clone()))
        })
        .collect();

    let math_query = meval::eval_str(query);

    if query.ends_with('+') ||
        query.ends_with('-') ||
        query.ends_with('*') ||
        query.ends_with('/') ||
        query.ends_with('(') ||
        query.ends_with(')') {

        // Handle incomplete expressions by trimming the last character
        let incomplete_expression = query[..query.len()-1].trim();
        if let Ok(result) = meval::eval_str(incomplete_expression) {
            Entry::math_result(result);
            scored.push((900i64, Entry::math_result(result)));
        }
    }
    // Otherwise handle complete expressions normally
    else if let Ok(result) = math_query {
        Entry::math_result(result);
        scored.push((1000i64, Entry::math_result(result)));
    }

    scored.sort_by(|a, b| b.0.cmp(&a.0));
    scored.into_iter().take(cfg.max_results).map(|(_, e)| e.clone()).collect()
}

// Unit tests
#[cfg(test)]
mod tests {
    use crate::entry::EntryKind;
    use super::*;

    #[test]
    // Test that empty query returns the first N entries
    fn empty_query_returns_first_n_entries() {
        let cfg = Arc::new(Config { max_results: 2, ..Default::default() });
        let entries = Rc::new(vec![
            Entry { title: "App1".into(), kind: EntryKind::Result(String::new()) },
            Entry { title: "App2".into(), kind: EntryKind::Result(String::new()) },
            Entry { title: "App3".into(), kind: EntryKind::Result(String::new()) },
        ]);
        let results = top_matches(&entries, "", cfg);
        assert_eq!(results.len(), 2);
    }

    #[test]
    // Test that math expression is evaluated correctly
    fn math_expression_evaluated() {
        let cfg = Arc::new(Config::default());
        let entries = Rc::new(vec![]);
        let results = top_matches(&entries, "2+2", cfg);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "4");
    }
}
