# dat

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rustc-1.77%2B-orange.svg)](https://www.rust-lang.org/)
[![Build](https://img.shields.io/github/actions/workflow/status/owner/dat/rust.yml?branch=main&label=build)](https://github.com/owner/dat/actions)
[![Crates.io](https://img.shields.io/crates/v/dat.svg)](https://crates.io/crates/dat)
[![Downloads](https://img.shields.io/crates/dv/dat.svg)](https://crates.io/crates/dat)

A terminal-based universal data viewer and manipulator for JSON, YAML, TOML, CSV, and TSV files.

Built in Rust with a full interactive TUI — think `jq` meets `less`, with a real terminal UI.

## Features

- **Universal format support** — JSON, YAML, TOML, CSV, and TSV files
- **Five interactive view modes** — tree, table, list, raw, and schema views
- **jq-like queries** — filter, sort, and navigate data with a familiar syntax
- **Six color themes** — dark, light, solarized, dracula, nord, and gruvbox
- **Command mode** — `:tree`, `:table`, `:theme dracula`, `:quit`, etc.
- **Filter & sort** — `/` to search, `Ctrl+f` to filter, `Ctrl+s` to sort
- **Read-only mode** — open files without modifying them

## Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Keybindings](#keybindings)
- [CLI Options](#cli-options)
- [Examples](#examples)
- [Development](#development)

## Installation

### From source

```bash
git clone https://github.com/iamzephlol/dat.git
cd dat
cargo build --release
```

The binary will be at `./target/release/dat`.

### Cargo

```bash
cargo install dat
```

### Pre-built binaries

Download the latest release from the [Releases page](https://github.com/iamzephlol/dat/releases).

## Quick Start

```bash
# Open a JSON file
dat data.json

# Open a CSV in table view
dat products.csv -f table

# Apply an initial query
dat config.json -q '.users[0].name'

# Pick a theme
dat config.yaml -t dracula
```

## Supported Formats

| Format | Extension | Auto-detection |
|--------|-----------|----------------|
| JSON   | `.json`   | ✅ |
| YAML   | `.yaml`, `.yml` | ✅ |
| TOML   | `.toml`   | ✅ |
| CSV    | `.csv`    | ✅ |
| TSV    | `.tsv`    | ✅ |

Format is auto-detected from the file extension. Use `-f/--format` to override.

## Keybindings

### View Modes

| Key | Action |
|-----|--------|
| `1` | Tree view |
| `2` | Table view |
| `3` | List view |
| `0` | Raw view |
| `s` | Schema view |
| `Tab` | Cycle view mode |

### Navigation & Control

| Key | Action |
|-----|--------|
| `q` / `Esc` | Quit |
| `?` / `h` | Toggle help |
| `/` | Search mode |
| `Ctrl+f` | Filter mode |
| `Ctrl+s` | Sort mode |
| `:` | Command mode |

### Command Mode (`:`)

| Command | Action |
|---------|--------|
| `:tree` | Switch to tree view |
| `:table` | Switch to table view |
| `:list` | Switch to list view |
| `:raw` | Switch to raw view |
| `:schema` | Switch to schema view |
| `:theme dark` | Change theme |
| `:theme dracula` | Change theme |
| `:quit` | Exit |

## CLI Options

```
dat [OPTIONS] [FILE]

ARGS:
    <FILE>    Input file to open

OPTIONS:
    -f, --format <FORMAT>   Display format: tree, table, list, raw, schema, auto [default: auto]
    -t, --theme <THEME>     Color theme: dark, light, solarized, dracula, nord, gruvbox [default: dark]
    -q, --query <QUERY>     Initial jq-like query
    -R, --readonly          Start in read-only mode
    -v, --verbose           Enable verbose logging
    -V, --version           Print version and exit
    -h, --help              Print help
```

## Examples

```bash
# Explore a JSON config file
dat ~/.config/myapp/config.json

# View a YAML file in schema mode
dat config.yaml -f schema

# Open a CSV in table view
dat products.csv -f table

# Filter to a specific path (jq-like syntax)
dat data.json -q '.users[0].name'

# Filter with array indexing
dat data.json -q '.users[] | select(.role == "admin")'

# Change theme
dat config.toml -t gruvbox

# Start in table view with a filter
dat products.csv -f table -q '.price'

# Open multiple files? Process from stdin
echo '{"test": [1, 2, 3]}' | dat -
```

## Development

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run with a test file
cargo run --release -- sample.json
```

### Running tests

```bash
cargo test
```

### Project Structure

```
dat/
├── Cargo.toml         # Package configuration
├── Cargo.lock         # Dependencies lockfile
├── LICENSE            # MIT License
├── README.md          # This file
├── sample.json        # Sample JSON file
├── users.json         # Sample JSON file
└── src/
    ├── main.rs        # CLI parsing, app entry point
    ├── app.rs         # App state, event loop, key handling
    ├── data.rs        # Value type, format loaders, query engine
    ├── input.rs       # Terminal input handling
    └── ui.rs          # Rendering, themes, view modes
```

### Dependencies

| Crate | Purpose |
|-------|---------|
| ratatui 0.25 | Terminal UI framework |
| crossterm 0.27 | Cross-platform terminal input |
| serde 1.0 | Serialization framework |
| serde_json 1.0 | JSON handling |
| serde_yaml 0.9 | YAML handling |
| toml 0.8 | TOML handling |
| csv 1.3 | CSV/TSV handling |
| clap 4 | CLI argument parsing |
| anyhow 1.0 | Error handling |

## Why "dat"?

Dat is short for **data** — it's the perfect name for a universal data viewer. It follows the Unix tradition of short, meaningful command names (`cat`, `ls`, `grep`, `jq`, `yq`), and works as both a noun and verb:

- "run dat on this file"
- "let me dat this JSON"

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

Distributed under the MIT License. See `LICENSE` for more information.

## Acknowledgments

- Inspired by `jq`, `yq`, and `bat`
- Built with `ratatui` and `crossterm`
- Theme colors inspired by popular terminal themes