use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{fs, io, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
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
    
    #[serde(default = "default_font_family")]
    pub font_family: String,
    
    #[serde(default = "default_bg_color")]
    pub bg_color: String,
    
    #[serde(default = "default_font_color")]
    pub font_color: String,
    
    #[serde(default = "default_selection_color")]
    pub selection_color: String,
    
    #[serde(default = "default_entry_min_height")]
    pub entry_min_height: u32,
    
    #[serde(default = "default_entry_border_color")]
    pub entry_border_color: String,
    
    #[serde(default = "default_entry_border_radius")]
    pub entry_border_radius: u32,
}

impl Default for Theme {
    fn default() -> Self {
        Self { 
            font_size: default_font_size(),
            font_family: default_font_family(),
            bg_color: default_bg_color(),
            font_color: default_font_color(),
            selection_color: default_selection_color(),
            entry_min_height: default_entry_min_height(),
            entry_border_color: default_entry_border_color(),
            entry_border_radius: default_entry_border_radius(),
        }
    }
}

fn default_show_icons() -> bool { true }
fn default_max_results() -> usize { 5 }
fn default_font_size() -> u32 { 14 }
fn default_terminal_emulator() -> String { "kitty".to_string() }
fn default_window_width() -> i32 { 600 }
fn default_window_height() -> i32 { 260 }
fn default_icon_size() -> i32 { 32 }
fn default_font_family() -> String { "Sans".to_string() }
fn default_bg_color() -> String { "#171717".to_string() }
fn default_font_color() -> String { "#f0f0f0".to_string() }
fn default_selection_color() -> String { "#1e46c9".to_string() }
fn default_entry_min_height() -> u32 { 32 }
fn default_entry_border_color() -> String { "#3a3a3a".to_string() }
fn default_entry_border_radius() -> u32 { 4 }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.show_icons);
        assert_eq!(config.max_results, 5);
        assert_eq!(config.theme.font_size, 14);
        assert_eq!(config.terminal_emulator, "kitty");
        assert_eq!(config.window_width, 600);
        assert_eq!(config.window_height, 260);
        assert_eq!(config.icon_size, 32);
    }

    #[test]
    fn test_default_theme() {
        let theme = Theme::default();
        assert_eq!(theme.font_size, 14);
    }

    #[test]
    fn test_parse_full_config_toml() {
        let toml_str = r#"
        show_icons = false
        max_results = 10
        terminal_emulator = "alacritty"
        window_width = 800
        window_height = 400
        icon_size = 48

        [theme]
        font_size = 16
        "#;

        let config: Config = toml::from_str(toml_str).expect("Failed to parse TOML");
        assert!(!config.show_icons);
        assert_eq!(config.max_results, 10);
        assert_eq!(config.terminal_emulator, "alacritty");
        assert_eq!(config.window_width, 800);
        assert_eq!(config.window_height, 400);
        assert_eq!(config.icon_size, 48);
        assert_eq!(config.theme.font_size, 16);
    }

    #[test]
    fn test_parse_partial_config_toml() {
        let toml_str = r#"
        show_icons = false
        max_results = 15
        "#;

        let config: Config = toml::from_str(toml_str).expect("Failed to parse TOML");
        assert!(!config.show_icons);
        assert_eq!(config.max_results, 15);
        // These should use defaults
        assert_eq!(config.terminal_emulator, "kitty");
        assert_eq!(config.window_width, 600);
        assert_eq!(config.window_height, 260);
        assert_eq!(config.icon_size, 32);
        assert_eq!(config.theme.font_size, 14);
    }

    #[test]
    fn test_parse_empty_config_toml() {
        let toml_str = "";

        let config: Config = toml::from_str(toml_str).expect("Failed to parse empty TOML");
        // All fields should use defaults
        assert!(config.show_icons);
        assert_eq!(config.max_results, 5);
        assert_eq!(config.terminal_emulator, "kitty");
        assert_eq!(config.window_width, 600);
        assert_eq!(config.window_height, 260);
        assert_eq!(config.icon_size, 32);
        assert_eq!(config.theme.font_size, 14);
    }

    #[test]
    fn test_parse_only_theme_toml() {
        let toml_str = r#"
        [theme]
        font_size = 20
        "#;

        let config: Config = toml::from_str(toml_str).expect("Failed to parse TOML");
        assert_eq!(config.theme.font_size, 20);
        // Other fields should use defaults
        assert!(config.show_icons);
        assert_eq!(config.max_results, 5);
        assert_eq!(config.terminal_emulator, "kitty");
    }

    #[test]
    fn test_serialize_config_to_toml() {
        let config = Config {
            show_icons: false,
            max_results: 8,
            theme: Theme { font_size: 18, ..Theme::default() },
            terminal_emulator: "gnome-terminal".to_string(),
            window_width: 700,
            window_height: 350,
            icon_size: 40,
        };

        let toml_str = toml::to_string_pretty(&config)
            .expect("Failed to serialize config to TOML");

        // Parse it back
        let parsed: Config = toml::from_str(&toml_str)
            .expect("Failed to parse serialized TOML");

        assert_eq!(config.show_icons, parsed.show_icons);
        assert_eq!(config.max_results, parsed.max_results);
        assert_eq!(config.theme.font_size, parsed.theme.font_size);
        assert_eq!(config.terminal_emulator, parsed.terminal_emulator);
        assert_eq!(config.window_width, parsed.window_width);
        assert_eq!(config.window_height, parsed.window_height);
        assert_eq!(config.icon_size, parsed.icon_size);
    }

    #[test]
    fn test_roundtrip_serialization() {
        let original = Config::default();

        let serialized = toml::to_string_pretty(&original)
            .expect("Failed to serialize");
        let deserialized: Config = toml::from_str(&serialized)
            .expect("Failed to deserialize");

        assert_eq!(original.show_icons, deserialized.show_icons);
        assert_eq!(original.max_results, deserialized.max_results);
        assert_eq!(original.theme.font_size, deserialized.theme.font_size);
        assert_eq!(original.terminal_emulator, deserialized.terminal_emulator);
        assert_eq!(original.window_width, deserialized.window_width);
        assert_eq!(original.window_height, deserialized.window_height);
        assert_eq!(original.icon_size, deserialized.icon_size);
    }

    #[test]
    fn test_invalid_toml_syntax() {
        let invalid_toml = r#"
        show_icons = false
        max_results = 10
        this is invalid toml
        "#;

        let result: Result<Config, _> = toml::from_str(invalid_toml);
        assert!(result.is_err(), "Should fail to parse invalid TOML");
    }

    #[test]
    fn test_invalid_type_conversion() {
        let invalid_toml = r#"
        show_icons = "not a boolean"
        "#;

        let result: Result<Config, _> = toml::from_str(invalid_toml);
        assert!(result.is_err(), "Should fail when boolean field has string value");
    }

    #[test]
    fn test_parse_max_results_as_integer() {
        let toml_str = r#"
        max_results = 20
        "#;

        let config: Config = toml::from_str(toml_str).expect("Failed to parse TOML");
        assert_eq!(config.max_results, 20);
    }

    #[test]
    fn test_parse_window_dimensions() {
        let toml_str = r#"
        window_width = 1024
        window_height = 768
        "#;

        let config: Config = toml::from_str(toml_str).expect("Failed to parse TOML");
        assert_eq!(config.window_width, 1024);
        assert_eq!(config.window_height, 768);
    }

    #[test]
    fn test_parse_terminal_emulator_string() {
        let toml_str = r#"
        terminal_emulator = "xterm"
        "#;

        let config: Config = toml::from_str(toml_str).expect("Failed to parse TOML");
        assert_eq!(config.terminal_emulator, "xterm");
    }

    #[test]
    fn test_config_clone() {
        let config = Config::default();
        let cloned = config.clone();

        assert_eq!(config.show_icons, cloned.show_icons);
        assert_eq!(config.max_results, cloned.max_results);
        assert_eq!(config.theme.font_size, cloned.theme.font_size);
    }

    #[test]
    fn test_theme_clone() {
        let theme = Theme { font_size: 18, ..Theme::default() };
        let cloned = theme.clone();

        assert_eq!(theme.font_size, cloned.font_size);
    }
}
