use clap::Parser;
use std::io::Result as IoResult;

mod data;
mod ui;
mod input;
mod app;

use crate::data::{load_data, Format};
use crate::app::App;

/// A terminal-based universal data viewer and manipulator
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Input file to open
    #[arg(value_name = "FILE")]
    pub file: Option<String>,

    /// Start in read-only mode
    #[arg(short = 'R', long = "readonly")]
    pub readonly: bool,

    /// Initial query/filter (jq-like syntax)
    #[arg(short = 'q', long = "query")]
    pub query: Option<String>,

    /// Display format: tree, table, list, raw, schema, auto
    #[arg(short = 'f', long = "format", default_value = "auto")]
    pub format: String,

    /// Color theme: dark, light, solarized, dracula, nord, gruvbox
    #[arg(short = 't', long = "theme", default_value = "dark")]
    pub theme: String,

    /// Enable verbose logging
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,

    /// Show version and exit
    #[arg(short = 'V', long = "version")]
    pub show_version: bool,
}

fn main() -> IoResult<()> {
    let args = Args::parse();

    if args.show_version {
        println!("dat {}", env!("CARGO_PKG_VERSION"));
        println!("A terminal-based universal data viewer");
        std::process::exit(0);
    }

    // Try to load file and start TUI
    if let Some(file) = &args.file {
        let path = std::path::Path::new(&file);
        let format = Format::from_path(path);

        match load_data(path, format) {
            Ok(value) => {
                // Start TUI
                match App::new(args.clone(), value, format) {
                    Ok(mut app) => {
                        if let Err(e) = app.run() {
                            eprintln!("Error running TUI: {}", e);
                            std::process::exit(1);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error initializing app: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error loading file: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        // No file - show help
        print_help();
    }

    Ok(())
}

fn print_help() {
    println!(
        r#"dat - A terminal-based universal data viewer

USAGE:
    dat [OPTIONS] [FILE]

DESCRIPTION:
    View and explore JSON, YAML, TOML, CSV, and TSV files in an interactive TUI.

OPTIONS:
    -f, --format <FORMAT>     Display format: tree, table, list, raw, schema, auto
    -t, --theme <THEME>       Color theme: dark, light, solarized, dracula, nord, gruvbox
    -q, --query <QUERY>       Initial jq-like query
    -R, --readonly            Start in read-only mode
    -v, --verbose             Enable verbose logging
    -V, --version             Print version and exit
    -h, --help                Print help

SUPPORTED FORMATS:
    JSON, YAML, TOML, CSV, TSV

EXAMPLES:
    dat data.json
    dat config.yaml --theme dracula
    dat products.csv --format table
    dat data.json --query '.users[0].name'
    dat config.toml --format schema

KEYBINDINGS:
    q or Esc          Quit
    1                 Tree view
    2                 Table view
    3                 List view
    0                 Raw view
    s                 Schema view
    Tab               Toggle view mode
    /                 Search (start typing after)
    Ctrl+f            Filter
    Ctrl+s            Sort
    ?                 Toggle help
    h                 Toggle help"#,
    );
}