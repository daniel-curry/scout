# Scout

A lightweight, fast application launcher for Linux built with Rust and GTK3.

## Overview

Scout is a minimal application launcher that provides quick access to your installed applications through a clean, keyboard-driven interface. It features fuzzy search to help you find and launch apps instantly.

## Features

- **Fuzzy Search**: Quickly find applications using fuzzy matching
- **Keyboard-Driven**: Navigate and launch apps without touching your mouse
- **Minimal UI**: Clean, undecorated window that stays focused on the task
- **Fast**: Built in Rust for optimal performance
- **Always on Top**: Window stays visible above other applications
- **Independent Process Management**: Launched apps run completely independent of Scout

## Installation

### Prerequisites

- Rust (latest stable version)
- GTK3 development libraries

On Debian/Ubuntu:
```bash
sudo apt install libgtk-3-dev
```

On Fedora:
```bash
sudo dnf install gtk3-devel
```

On Arch Linux:
```bash
sudo pacman -S gtk3
```

### Building from Source

1. Clone the repository:
```bash
git clone https://github.com/daniel-curry/scout.git
cd scout
```

2. Build the project:
```bash
cargo build --release
```

3. The binary will be available at `target/release/scout`

4. (Optional) Install to your system:
```bash
sudo cp target/release/scout /usr/local/bin/
```

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
- When no search query is entered, Scout displays your most recently installed applications

## Configuration

Scout uses the system's application database (`.desktop` files) to discover installed applications. No additional configuration is needed.

## How It Works

Scout scans your system's application database and presents a searchable interface using:

- **GTK3**: For the user interface
- **fuzzy-matcher**: For intelligent search matching
- **GIO/GLib**: For application discovery and management

## Development

### Project Structure

```
scout/
├── src/
│   └── main.rs          # Main application code
├── Cargo.toml           # Project dependencies
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

## License

This project is licensed under the MIT License. 

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Roadmap

Potential future enhancements:
- Custom keybindings
- Icon display in results
- Theme customization
- Plugin support
- External API support
