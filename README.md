# Scout

A lightweight, fast application launcher for Linux built with Rust and GTK3.

## Overview

Scout is a minimal application launcher that provides quick access to your installed applications through a clean, keyboard-driven interface. It features fuzzy search to help you find and launch apps instantly.

## Features

- **Fuzzy Search**: Quickly find applications using fuzzy matching
- **Keyboard-Driven**: Navigate and launch apps without touching your mouse
- **Fast**: Built in Rust for optimal performance
- **Terminal App Support**: Automatically launch terminal applications in your preferred terminal emulator
- **Built-In Calculator**: Easily get calculations right from the launcher
- **System Actions**: Quick access to Shutdown, Restart, Sleep, and Hibernate from the launcher
- **Configurable**: Customize Scout via a TOML configuration file

## Installation

### Dependencies

#### Arch / EndeavourOS
```bash
sudo pacman -S --needed gtk3 pkgconf
```

#### Debian/Ubuntu
```bash
sudo apt update
sudo apt install -y libgtk-3-dev pkg-config
```

#### Fedora
```bash
sudo dnf update
sudo dnf install -y gtk3-devel pkgconf-pkg-config
```


### Building from Source

Clone the repository:
```bash
cargo install --git https://github.com/daniel-curry/scout --locked
````

## Usage

### Running Scout

Simply run the executable:
```bash
scout
```

### Keyboard Shortcuts

- **Type**: Start typing to search for applications
- **↑/↓**: Navigate through search results
- **Enter**: Launch the selected application
- **Escape**: Close Scout

### Tips

- The search is fuzzy, so you don't need to type exact names (e.g., "fir" will match "Firefox")
- When no search query is entered, Scout displays applications from your system (up to the configured `max_results`)
- Type "shutdown", "restart", "sleep", or "hibernate" to access system power actions

## Configuration

Scout is fully configurable via a TOML configuration file. On first run, a default configuration file is created at:

```
~/.config/scout/config.toml
```

### Configuration Options

| Option                       | Type    | Default       | Description                                        |
|------------------------------|---------|---------------|----------------------------------------------------|
| `show_icons`                 | boolean | `true`        | Enable/disable application icons in search results |
| `max_results`                | integer | `5`           | Maximum number of search results to display        |
| `terminal_emulator`          | string  | `"kitty"`     | Terminal emulator to use for terminal applications |
| `window_width`               | integer | `600`         | Width of the Scout window in pixels                |
| `window_height`              | integer | `260`         | Height of the Scout window in pixels               |
| `icon_size`                  | integer | `32`          | Size of application icons in pixels                |
| `theme.font_size`            | integer | `14`          | Font size for UI elements                          |
| `theme.font_family`          | string  | `"Sans"`      | Font family for UI elements                        |
| `theme.bg_color`             | string  | `"#171717"`   | Background color of the window                     |
| `theme.font_color`           | string  | `"#f0f0f0"`   | Color of text elements                             |
| `theme.selection_color`      | string  | `"#1e46c9"`   | Background color for selected items                |
| `theme.entry_min_height`     | integer | `32`          | Minimum height of the search entry box in pixels   |
| `theme.entry_border_color`   | string  | `"#3a3a3a"`   | Border color of the search entry box               |
| `theme.entry_border_radius`  | integer | `4`           | Border radius of the search entry box in pixels    |

### Example Configuration

```toml
show_icons = true
max_results = 5
terminal_emulator = "kitty"
window_width = 600
window_height = 260
icon_size = 32

[theme]
font_size = 14
font_family = "Sans"
bg_color = "#171717"
font_color = "#f0f0f0"
selection_color = "#1e46c9"
entry_min_height = 32
entry_border_color = "#3a3a3a"
entry_border_radius = 4
```

## How It Works

Scout scans your system's application database and presents a searchable interface using:

- **GTK3**: For the user interface
- **fuzzy-matcher**: For intelligent search matching
- **GIO/GLib**: For application discovery and management
- **serde/toml**: For configuration file parsing
- **directories**: For cross-platform config file location

## Development

### Project Structure

```
scout/
├── src/
│   ├── main.rs          # Application entry point
│   ├── app.rs           # GTK application setup
│   ├── config.rs        # Configuration loading and defaults
│   ├── entry.rs         # Entry types (apps and system actions)
│   ├── icon.rs          # Icon loading and rendering
│   ├── launcher.rs      # Application and action launching
│   ├── search.rs        # Fuzzy search implementation
│   └── ui.rs            # UI building and event handling
├── Cargo.toml           # Project dependencies
├── Cargo.lock           # Dependency lock file
├── LICENSE              # MIT License
└── README.md            # This file
```

### Running in Development Mode

```bash
cargo run
```

### Building

```bash
cargo build --release
```

## Dependencies

- `gtk` (0.18.2) - GTK3 bindings for Rust
- `gdk` (0.18.2) - GDK bindings
- `gio` (0.18) - GIO for application info
- `glib` (0.18) - GLib utilities
- `fuzzy-matcher` (0.3.7) - Fuzzy string matching
- `libc` (0.2) - C library bindings for process management
- `system_shutdown` (4.0) - System power actions (shutdown, restart, etc.)
- `serde` (1.0) - Serialization/deserialization framework
- `toml` (0.8) - TOML configuration file parsing
- `directories` (5.0) - Cross-platform config directory paths
- `meval` (0.2) - Mathematical expression evaluation for calculator

## License

This project is licensed under the MIT License. 

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
