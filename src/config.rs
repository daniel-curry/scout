use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{fs, io, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    //TODO: Add functionality to enable/disable icons
    #[serde(default = "default_show_icons")]
    pub show_icons: bool,

    #[serde(default = "default_max_results")]
    pub max_results: usize,

    #[serde(default)]
    pub theme: Theme,

    #[serde(default = "default_terminal_emulator")]
    pub terminal_emulator: String,

    #[serde(default = "default_window_width")]
    pub window_width: i32,

    #[serde(default = "default_window_height")]
    pub window_height: i32,

    #[serde(default = "default_icon_size")]
    pub icon_size: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    #[serde(default = "default_font_size")]
    pub font_size: u32,
}

impl Default for Theme {
    fn default() -> Self {
        Self { font_size: default_font_size() }
    }
}

fn default_show_icons() -> bool { true }
fn default_max_results() -> usize { 5 }
fn default_font_size() -> u32 { 14 }
fn default_terminal_emulator() -> String { "kitty".to_string() }
fn default_window_width() -> i32 { 600 }
fn default_window_height() -> i32 { 260 }
fn default_icon_size() -> i32 { 32 }

impl Default for Config {
    fn default() -> Self {
        Self {
            show_icons: default_show_icons(),
            max_results: default_max_results(),
            theme: Theme::default(),
            terminal_emulator: default_terminal_emulator(),
            window_width: default_window_width(),
            window_height: default_window_height(),
            icon_size: default_icon_size(),
        }
    }
}

fn config_path() -> io::Result<PathBuf> {
    // (qualifier, organization, application)
    let proj = ProjectDirs::from("io", "daniel-curry", "scout")
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "cannot determine config dir"))?;

    Ok(proj.config_dir().join("config.toml"))
}

pub fn load_or_create() -> io::Result<Config> {
    let path = config_path()?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    if !path.exists() {
        // First run: write a default config to disk so the user can edit it.
        let default_cfg = Config::default();
        let s = toml::to_string_pretty(&default_cfg)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(&path, s)?;
        return Ok(default_cfg);
    }

    let raw = fs::read_to_string(&path)?;
    let cfg: Config = toml::from_str(&raw)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(cfg)
}
