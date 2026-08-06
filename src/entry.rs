use crate::icon::{create_app_icon_widget, create_generic_icon_widget};
use crate::launcher::launch_system_action;
use gio::{AppInfo, prelude::AppInfoExt};

use crate::config::Config;
use crate::launcher::{launch_gui_app, launch_terminal_application, needs_terminal};
use gtk::{Application, Image, prelude::*};

pub enum ActivateAction {
    Dismiss,
    KeepOpen,
}

pub trait Launch {
    fn title(&self) -> String;
    fn icon(&self, cfg: &Config) -> Image;

    fn activate(&self, _cfg: &Config) -> ActivateAction {
        ActivateAction::Dismiss
    }
}

pub struct AppEntry {
    pub app: Application,
    pub info: AppInfo,
}

impl Launch for AppEntry {
    fn title(&self) -> String {
        self.info.display_name().to_string()
    }
    fn icon(&self, cfg: &Config) -> Image {
        create_app_icon_widget(&self.info, cfg.clone())
    }
    fn activate(&self, cfg: &Config) -> ActivateAction {
        let app_clone = self.app.clone();
        if needs_terminal(&self.info) {
            let exec = self.info.executable().to_string_lossy().into_owned();
            let term = cfg.terminal_emulator.to_string();
            launch_terminal_application(&[exec], &[term])
                .map_err(|e| format!("Failed to launch terminal app: {}", e))
                .unwrap_or_else(|err| eprintln!("Launch failed: {err}"));
            app_clone.quit();
            return ActivateAction::Dismiss;
        }

        if let Err(err) = launch_gui_app(&self.info) {
            eprintln!("Launch failed: {err}");
            ActivateAction::Dismiss
        } else {
            self.app.clone().quit();
            ActivateAction::Dismiss
        }
    }
}

#[derive(Clone, Debug)]
pub enum SystemAction {
    Shutdown,
    Restart,
    Hibernate,
    Sleep,
}

pub struct SystemPowerEntry {
    pub(crate) action: SystemAction,
}

impl Launch for SystemPowerEntry {
    fn title(&self) -> String {
        match &self.action {
            SystemAction::Shutdown => String::from("Shutdown"),
            SystemAction::Restart => String::from("Restart"),
            SystemAction::Hibernate => String::from("Hibernate"),
            SystemAction::Sleep => String::from("Sleep"),
        }
    }
    fn icon(&self, cfg: &Config) -> Image {
        create_generic_icon_widget("system-shutdown", cfg.clone())
    }

    fn activate(&self, _cfg: &Config) -> ActivateAction {
        launch_system_action(&self.action).expect("System action failed");
        ActivateAction::Dismiss
    }
}

pub struct MathResultEntry {
    pub(crate) expr: String,
}
impl Launch for MathResultEntry {
    fn title(&self) -> String {
        format!("{}", self.expr)
    }
    fn icon(&self, cfg: &Config) -> Image {
        create_generic_icon_widget("calculator", cfg.clone())
    }
}
