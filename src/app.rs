use anyhow::Result;
use std::io;
use std::time::Duration;

use crate::data::{Format, Value, ViewMode};
use crate::input::InputHandler;
use crate::ui::{Renderer, Theme, colors::ThemeColors};

/// Main application state
pub struct App {
    /// Terminal UI
    terminal: ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
    /// Input handler
    input: InputHandler,
    /// Renderer
    renderer: Renderer,
    /// Current data
    data: Value,
    /// File format
    _format: Format,
    /// Current view mode
    view_mode: ViewMode,
    /// Current theme
    theme: Theme,
    /// Theme colors
    theme_colors: ThemeColors,
    /// Current query/filter
    query: String,
    /// Whether we're in filter mode
    filter_mode: bool,
    /// Whether we're in search mode
    search_mode: bool,
    /// Whether we're in sort mode
    sort_mode: bool,
    /// Whether we're in command mode
    command_mode: bool,
    /// Current command buffer
    command: String,
    /// Whether to show help
    show_help: bool,
    /// Whether to quit
    should_quit: bool,
    /// Status message
    status: String,
}

impl App {
    /// Create new app with initial data
    pub fn new(args: crate::Args, data: Value, format: Format) -> Result<Self> {
        // Setup terminal
        let backend = ratatui::backend::CrosstermBackend::new(io::stdout());
        let terminal = ratatui::Terminal::new(backend)?;

        // Setup input handler
        let input = InputHandler::new();

        // Setup theme
        let theme = Theme::from_str(&args.theme);
        let theme_colors = theme.get_colors();

        Ok(Self {
            terminal,
            input,
            renderer: Renderer::new(),
            data,
            _format: format,
            view_mode: ViewMode::Tree,
            theme,
            theme_colors,
            query: args.query.unwrap_or_default(),
            filter_mode: false,
            search_mode: false,
            sort_mode: false,
            command_mode: false,
            command: String::new(),
            show_help: false,
            should_quit: false,
            status: String::new(),
        })
    }

    /// Run the main event loop
    pub fn run(&mut self) -> io::Result<()> {
        // Enter alternate screen
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(io::stdout(), crossterm::terminal::EnterAlternateScreen)?;

        // Hide cursor
        self.terminal.hide_cursor()?;

        // Main loop
        while !self.should_quit {
            // Draw
            self.draw()?;

            // Handle input
            self.handle_events()?;

            // Small sleep to avoid busy-waiting
            std::thread::sleep(Duration::from_millis(16));
        }

        // Cleanup
        self.terminal.show_cursor()?;
        crossterm::execute!(io::stdout(), crossterm::terminal::LeaveAlternateScreen)?;
        crossterm::terminal::disable_raw_mode()?;

        Ok(())
    }

    /// Draw the UI
    fn draw(&mut self) -> io::Result<()> {
        let mut frame = self.terminal.get_frame();
        let area = frame.size();

        self.renderer.render(
            &mut frame,
            area,
            &self.data,
            self.view_mode,
            &self.theme_colors,
            self.show_help,
            &self.query,
            self.filter_mode || self.search_mode || self.command_mode,
            &self.command,
            &self.status,
        );

        self.terminal.show_cursor()?;
        self.terminal.flush()?;

        Ok(())
    }

    /// Handle input events
    fn handle_events(&mut self) -> io::Result<()> {
        // Check for input
        if let Some(event) = self.input.try_read() {
            match event {
                crate::input::InputEvent::Key(key_event) => {
                    self.handle_key(key_event)?;
                }
                crate::input::InputEvent::Resize(cols, rows) => {
                    self.terminal.resize(ratatui::layout::Rect::new(0, 0, cols, rows))?;
                }
            }
        }
        Ok(())
    }

    /// Handle key events
    fn handle_key(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        use crossterm::event::{KeyCode, KeyModifiers};

        // Handle command mode
        if self.command_mode {
            return self.handle_command_mode(key_event);
        }

        // Handle filter mode
        if self.filter_mode || self.search_mode {
            return self.handle_filter_mode(key_event);
        }

        // Handle sort mode
        if self.sort_mode {
            return self.handle_sort_mode(key_event);
        }

        // Normal mode key handling
        match key_event.code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Esc => {
                if self.show_help {
                    self.show_help = false;
                } else {
                    self.should_quit = true;
                }
            }
            KeyCode::Char('h') | KeyCode::Char('?') => {
                self.show_help = !self.show_help;
            }
            KeyCode::Char('1') => self.view_mode = ViewMode::Tree,
            KeyCode::Char('2') => self.view_mode = ViewMode::Table,
            KeyCode::Char('3') => self.view_mode = ViewMode::List,
            KeyCode::Char('0') => self.view_mode = ViewMode::Raw,
            KeyCode::Char('s') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.sort_mode = true;
                self.command = String::new();
                self.status = "Sort field: ".to_string();
            }
            KeyCode::Char('s') => self.view_mode = ViewMode::Schema,
            KeyCode::Tab => self.cycle_view_mode(),
            KeyCode::Char(':') => {
                self.command_mode = true;
                self.command = String::new();
                self.status = ":".to_string();
            }
            _ => {}
        }
        Ok(())
    }

    /// Cycle view mode
    fn cycle_view_mode(&mut self) {
        self.view_mode = match self.view_mode {
            ViewMode::Tree => ViewMode::Table,
            ViewMode::Table => ViewMode::List,
            ViewMode::List => ViewMode::Raw,
            ViewMode::Raw => ViewMode::Schema,
            ViewMode::Schema => ViewMode::Tree,
        };
    }

    /// Handle filter/search input
    fn handle_filter_mode(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Esc => {
                self.filter_mode = false;
                self.search_mode = false;
                self.command.clear();
                self.status.clear();
            }
            KeyCode::Enter => {
                // Apply filter
                self.apply_filter();
                self.filter_mode = false;
                self.search_mode = false;
                self.command.clear();
                self.status.clear();
            }
            KeyCode::Backspace => {
                self.command.pop();
                self.status = format!("Filter: {}", self.command);
            }
            KeyCode::Char(c) => {
                self.command.push(c);
                self.status = format!("Filter: {}", self.command);
            }
            _ => {}
        }
        Ok(())
    }

    /// Apply the current filter
    fn apply_filter(&mut self) {
        if self.command.is_empty() {
            return;
        }

        // Try to parse as jq-like query
        match crate::data::query::QueryEngine::new().query(&self.data, &self.command) {
            Ok(result) => {
                self.data = result;
                self.status = format!("Applied filter: {}", self.command);
            }
            Err(e) => {
                self.status = format!("Filter error: {}", e);
            }
        }
    }

    /// Handle sort mode
    fn handle_sort_mode(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Esc => {
                self.sort_mode = false;
                self.command.clear();
                self.status.clear();
            }
            KeyCode::Enter => {
                self.apply_sort();
                self.sort_mode = false;
                self.command.clear();
                self.status.clear();
            }
            KeyCode::Backspace => {
                self.command.pop();
                self.status = format!("Sort field: {}", self.command);
            }
            KeyCode::Char(c) => {
                self.command.push(c);
                self.status = format!("Sort field: {}", self.command);
            }
            _ => {}
        }
        Ok(())
    }

    /// Apply sorting
    fn apply_sort(&mut self) {
        if self.command.is_empty() {
            return;
        }

        match crate::data::query::QueryEngine::new().sort_array(
            self.data.as_array().unwrap_or(&Vec::new()).to_vec(),
            &self.command,
            true
        ) {
            sorted => {
                if let Value::Array(arr) = &mut self.data {
                    *arr = sorted;
                }
                self.status = format!("Sorted by: {}", self.command);
            }
        }
    }

    /// Handle command mode (: commands)
    fn handle_command_mode(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Esc => {
                self.command_mode = false;
                self.command.clear();
                self.status.clear();
            }
            KeyCode::Enter => {
                self.execute_command();
                self.command_mode = false;
                self.command.clear();
                self.status.clear();
            }
            KeyCode::Backspace => {
                self.command.pop();
                self.status = format!(":{}", self.command);
            }
            KeyCode::Char(c) => {
                self.command.push(c);
                self.status = format!(":{}", self.command);
            }
            _ => {}
        }
        Ok(())
    }

    /// Execute a command
    fn execute_command(&mut self) {
        let cmd = self.command.trim();
        match cmd {
            "q" | "quit" | "exit" => self.should_quit = true,
            "h" | "help" => self.show_help = true,
            "tree" => self.view_mode = ViewMode::Tree,
            "table" => self.view_mode = ViewMode::Table,
            "list" => self.view_mode = ViewMode::List,
            "raw" => self.view_mode = ViewMode::Raw,
            "schema" => self.view_mode = ViewMode::Schema,
            "theme" => self.cycle_theme(),
            _ if cmd.starts_with("theme ") => {
                let theme_name = cmd.strip_prefix("theme ").unwrap_or("");
                self.theme = Theme::from_str(theme_name);
                self.theme_colors = self.theme.get_colors();
                self.status = format!("Theme: {}", theme_name);
            }
            _ => self.status = format!("Unknown command: {}", cmd),
        }
    }

    /// Cycle through themes
    fn cycle_theme(&mut self) {
        self.theme = match self.theme {
            Theme::Dark => Theme::Light,
            Theme::Light => Theme::SolarizedDark,
            Theme::SolarizedDark => Theme::Dracula,
            Theme::Dracula => Theme::Nord,
            Theme::Nord => Theme::Gruvbox,
            Theme::Gruvbox => Theme::Dark,
            Theme::SolarizedLight => Theme::Dark,
        };
        self.theme_colors = self.theme.get_colors();
        self.status = format!("Theme: {:?}", self.theme);
    }
}