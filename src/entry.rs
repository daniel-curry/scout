use gio::AppInfo;

#[derive(Clone, Debug)]
pub enum EntryKind {
    App(AppInfo),
    Action(SystemAction),
    Result(String),
}

#[derive(Clone, Debug)]
pub struct Entry {
    pub title: String,
    pub kind: EntryKind,
}

#[derive(Clone, Debug)]
pub enum SystemAction {
    Shutdown,
    Restart,
    Hibernate,
    Sleep,
}


impl Entry {
    pub fn from_app(app: AppInfo) -> Self {
        use gio::prelude::AppInfoExt;
        Self {
            title: app.display_name().to_string(),
            kind: EntryKind::App(app),
        }
    }

    pub fn system_action(action: SystemAction) -> Self {
        let title = match &action {
            SystemAction::Shutdown => "Shutdown",
            SystemAction::Restart => "Restart",
            SystemAction::Sleep => "Sleep",
            SystemAction::Hibernate => "Hibernate",
        };

        Self {
            title: title.to_string(),
            kind: EntryKind::Action(action),
        }
    }

    pub fn math_result(result: f64) -> Self {
        Self {
            title: format!("{}", result),
            kind: EntryKind::Result(String::new()),
        }
    }
}
