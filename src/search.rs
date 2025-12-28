use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use gio::AppInfo;
use gio::prelude::AppInfoExt;

pub fn get_apps() -> Vec<AppInfo> {
    AppInfo::all()
        .into_iter()
        .filter(|a| a.should_show())
        .collect()
}

pub fn top_matches(apps: &[AppInfo], query: &str, k: usize) -> Vec<AppInfo> {
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