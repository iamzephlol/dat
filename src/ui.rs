use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, TableState, Wrap,
    },
    Frame,
};

use crate::data::{Value, ViewMode};
use crate::ui::colors::ThemeColors;

/// Main renderer
pub struct Renderer {
    list_state: ListState,
    table_state: TableState,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            list_state: ListState::default(),
            table_state: TableState::default(),
        }
    }

    pub fn render(
        &mut self,
        frame: &mut Frame<'_>,
        area: Rect,
        data: &Value,
        view_mode: ViewMode,
        theme_colors: &ThemeColors,
        show_help: bool,
        query: &str,
        in_filter_mode: bool,
        command: &str,
        status: &str,
    ) {
        // Main layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(1),    // Content
                Constraint::Length(1), // Status bar
            ])
            .split(area);

        // Header
        self.render_header(frame, chunks[0], view_mode, theme_colors, query);

        // Content
        if show_help {
            self.render_help(frame, chunks[1], theme_colors);
        } else {
            match view_mode {
                ViewMode::Tree => self.render_tree(frame, chunks[1], data, theme_colors),
                ViewMode::Table => self.render_table(frame, chunks[1], data, theme_colors),
                ViewMode::List => self.render_list(frame, chunks[1], data, theme_colors),
                ViewMode::Raw => self.render_raw(frame, chunks[1], data, theme_colors),
                ViewMode::Schema => self.render_schema(frame, chunks[1], data, theme_colors),
            }
        }

        // Status bar
        self.render_status_bar(frame, chunks[2], theme_colors, in_filter_mode, command, status);
    }

    fn render_status_bar(
        &mut self,
        frame: &mut Frame<'_>,
        area: Rect,
        theme_colors: &ThemeColors,
        in_filter_mode: bool,
        command: &str,
        status: &str,
    ) {
        let text = if in_filter_mode && !command.is_empty() {
            format!("{} {}", command, status)
        } else if !status.is_empty() {
            status.to_string()
        } else {
            "dat — press ? for help".to_string()
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme_colors.border));

        let paragraph = Paragraph::new(text)
            .block(block)
            .style(Style::default().fg(theme_colors.dim));

        frame.render_widget(paragraph, area);
    }

    fn render_header(
        &mut self,
        frame: &mut Frame<'_>,
        area: Rect,
        view_mode: ViewMode,
        theme_colors: &ThemeColors,
        query: &str,
    ) {
        let title = Line::from(vec![
            Span::styled("dat ", Style::default().fg(theme_colors.accent).add_modifier(Modifier::BOLD)),
            Span::styled(format!("v{} ", env!("CARGO_PKG_VERSION")), Style::default().fg(theme_colors.dim)),
            Span::styled(format!(" [{:?}] ", view_mode), Style::default().fg(theme_colors.header)),
        ]);

        if !query.is_empty() {
            let query_span = Line::from(vec![
                Span::styled("Filter: ", Style::default().fg(theme_colors.warning)),
                Span::styled(query.to_string(), Style::default().fg(theme_colors.foreground)),
            ]);
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(20),
                    Constraint::Min(0),
                ])
                .split(area);

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme_colors.border))
                .title(title)
                .title_alignment(Alignment::Center);

            let paragraph = Paragraph::new(query_span)
                .block(Block::default().borders(Borders::NONE));

            frame.render_widget(block, chunks[0]);
            frame.render_widget(paragraph, chunks[1]);
        } else {
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme_colors.border))
                .title(title)
                .title_alignment(Alignment::Center);

            frame.render_widget(Paragraph::new("").block(block), area);
        }
    }

    fn render_help(
        &mut self,
        frame: &mut Frame<'_>,
        area: Rect,
        theme_colors: &ThemeColors,
    ) {
        let help_text = vec![
            Line::from(Span::styled("dat Help", Style::default().add_modifier(Modifier::BOLD))),
            Line::from(""),
            Line::from("NAVIGATION:"),
            Line::from("  ↑ / k      Move cursor up"),
            Line::from("  ↓ / j      Move cursor down"),
            Line::from("  ← / h      Move cursor left"),
            Line::from("  → / l      Move cursor right"),
            Line::from("  PgUp       Page up"),
            Line::from("  PgDn       Page down"),
            Line::from("  Home       Go to start"),
            Line::from("  End        Go to end"),
            Line::from(""),
            Line::from("VIEWS:"),
            Line::from("  1          Tree view"),
            Line::from("  2          Table view"),
            Line::from("  3          List view"),
            Line::from("  0          Raw view"),
            Line::from("  s          Schema view"),
            Line::from("  Tab        Cycle through views"),
            Line::from(""),
            Line::from("ACTIONS:"),
            Line::from("  Enter      Expand/collapse or edit"),
            Line::from("  Space      Toggle selection"),
            Line::from("  /          Start filter/search"),
            Line::from("  Ctrl+f     Advanced filter (jq-like)"),
            Line::from("  Ctrl+s     Sort by field"),
            Line::from("  d          Delete item"),
            Line::from("  y          Yank (copy)"),
            Line::from("  p          Paste"),
            Line::from("  r          Rename key"),
            Line::from("  :          Command mode"),
            Line::from("  ? / h      Toggle this help"),
            Line::from("  q / Esc    Quit"),
            Line::from(""),
            Line::from("COMMANDS (type : then command):"),
            Line::from("  q, quit    Quit"),
            Line::from("  help       Show this help"),
            Line::from("  tree       Switch to tree view"),
            Line::from("  table      Switch to table view"),
            Line::from("  list       Switch to list view"),
            Line::from("  raw        Switch to raw view"),
            Line::from("  schema     Switch to schema view"),
            Line::from("  theme      Cycle theme"),
            Line::from("  theme <name> Set theme (dark, light, solarized, dracula, nord, gruvbox)"),
        ];

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme_colors.border))
            .title("Help")
            .title_alignment(Alignment::Center);

        let paragraph = Paragraph::new(help_text)
            .block(block);

        frame.render_widget(paragraph, area);
    }

    fn render_tree(
        &mut self,
        frame: &mut Frame<'_>,
        area: Rect,
        data: &Value,
        theme_colors: &ThemeColors,
    ) {
        let items = value_to_tree_items(data, "", 0);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme_colors.border))
            .title("Tree View")
            .title_alignment(Alignment::Center);

        let list = List::new(items)
            .block(block)
            .highlight_style(
                Style::default()
                    .bg(theme_colors.selected)
                    .fg(Color::White),
            )
            .highlight_symbol("▸ ");

        frame.render_stateful_widget(list, area, &mut self.list_state);
    }

    fn render_table(
        &mut self,
        frame: &mut Frame<'_>,
        area: Rect,
        data: &Value,
        theme_colors: &ThemeColors,
    ) {
        let (headers, rows) = value_to_table_data(data);

        if headers.is_empty() && rows.is_empty() {
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme_colors.border))
                .title("Table View")
                .title_alignment(Alignment::Center);

            let paragraph = Paragraph::new(vec![Line::from("No data to display")])
                .block(block);
            frame.render_widget(paragraph, area);
            return;
        }

        let widths: Vec<Constraint> = headers
            .iter()
            .map(|_| Constraint::Percentage(100 / headers.len() as u16))
            .collect();

        let table_rows: Vec<Row> = rows
            .iter()
            .map(|row| {
                let cells: Vec<Cell> = row.iter()
                    .map(|cell| Cell::from(cell.clone()))
                    .collect();
                Row::new(cells)
            })
            .collect();

        let header_cells: Vec<Cell> = headers
            .iter()
            .map(|h| {
                Cell::from(h.as_str())
                    .style(Style::default()
                        .fg(theme_colors.header)
                        .add_modifier(Modifier::BOLD))
            })
            .collect();

        let table = Table::new(table_rows, widths)
            .header(Row::new(header_cells))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(theme_colors.border))
                    .title("Table View")
                    .title_alignment(Alignment::Center),
            )
            .highlight_style(
                Style::default()
                    .bg(theme_colors.selected)
                    .fg(Color::White),
            )
            .column_spacing(2);

        frame.render_stateful_widget(table, area, &mut self.table_state);
    }

    fn render_list(
        &mut self,
        frame: &mut Frame<'_>,
        area: Rect,
        data: &Value,
        theme_colors: &ThemeColors,
    ) {
        let items = value_to_list_items(data);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme_colors.border))
            .title("List View")
            .title_alignment(Alignment::Center);

        let list = List::new(items)
            .block(block)
            .highlight_style(
                Style::default()
                    .bg(theme_colors.selected)
                    .fg(Color::White),
            )
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, area, &mut self.list_state);
    }

    fn render_raw(
        &mut self,
        frame: &mut Frame<'_>,
        area: Rect,
        data: &Value,
        theme_colors: &ThemeColors,
    ) {
        let text = format_raw(data);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme_colors.border))
            .title("Raw View")
            .title_alignment(Alignment::Center);

        let paragraph = Paragraph::new(text)
            .block(block)
            .style(Style::default().fg(theme_colors.foreground))
            .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, area);
    }

    fn render_schema(
        &mut self,
        frame: &mut Frame<'_>,
        area: Rect,
        data: &Value,
        theme_colors: &ThemeColors,
    ) {
        let text = value_to_schema_text(data);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme_colors.border))
            .title("Schema View")
            .title_alignment(Alignment::Center);

        let paragraph = Paragraph::new(text)
            .block(block)
            .style(Style::default().fg(theme_colors.foreground))
            .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, area);
    }
}

/// Free helper functions (outside impl Renderer to avoid borrow conflicts)

fn value_to_tree_items(data: &Value, prefix: &str, depth: usize) -> Vec<ListItem<'static>> {
    let mut items = Vec::new();

    match data {
        Value::Null => items.push(ListItem::new(format!("{}null", prefix))),
        Value::Bool(b) => items.push(ListItem::new(format!("{}bool: {}", prefix, b))),
        Value::Integer(i) => items.push(ListItem::new(format!("{}int: {}", prefix, i))),
        Value::Float(f) => items.push(ListItem::new(format!("{}float: {}", prefix, f))),
        Value::String(s) => {
            let display = if s.len() > 40 {
                format!("\"{}\"...", &s[..40])
            } else {
                format!("\"{}\"", s)
            };
            items.push(ListItem::new(format!("{}string: {}", prefix, display)));
        }
        Value::Array(arr) => {
            items.push(ListItem::new(format!("{}array ({} items)", prefix, arr.len())));
            for (i, value) in arr.iter().enumerate().take(10) {
                let child_prefix = format!("{}  [{:>3}] ", prefix, i);
                items.extend(value_to_tree_items(value, &child_prefix, depth + 1));
            }
            if arr.len() > 10 {
                items.push(ListItem::new(format!("{}  ... and {} more", prefix, arr.len() - 10)));
            }
        }
        Value::Object(obj) => {
            items.push(ListItem::new(format!("{}object ({} keys)", prefix, obj.len())));
            for (_i, (key, value)) in obj.iter().enumerate().take(10) {
                let child_prefix = format!("{}  \"{}\": ", prefix, key);
                items.extend(value_to_tree_items(value, &child_prefix, depth + 1));
            }
            if obj.len() > 10 {
                items.push(ListItem::new(format!("{}  ... and {} more", prefix, obj.len() - 10)));
            }
        }
    }

    items
}

fn value_to_table_data(data: &Value) -> (Vec<String>, Vec<Vec<String>>) {
    let mut headers = Vec::new();
    let mut rows = Vec::new();

    match data {
        Value::Array(arr) => {
            if arr.is_empty() {
                return (headers, rows);
            }

            if let Some(first) = arr.first() {
                if let Value::Object(obj) = first {
                    headers = obj.iter().map(|(k, _)| k.clone()).collect();
                    for value in arr {
                        let mut row = Vec::new();
                        if let Value::Object(obj) = value {
                            for key in &headers {
                                let cell = obj
                                    .iter()
                                    .find(|(k, _)| k == key)
                                    .map(|(_, v)| value_to_string(v))
                                    .unwrap_or_else(|| "-".to_string());
                                row.push(cell);
                            }
                        }
                        rows.push(row);
                    }
                } else {
                    headers = vec!["value".to_string()];
                    for value in arr {
                        rows.push(vec![value_to_string(value)]);
                    }
                }
            }
        }
        Value::Object(obj) => {
            headers = vec!["key".to_string(), "value".to_string()];
            for (key, value) in obj {
                rows.push(vec![
                    key.clone(),
                    value_to_string(value),
                ]);
            }
        }
        _ => {
            headers = vec!["value".to_string()];
            rows.push(vec![value_to_string(data)]);
        }
    }

    (headers, rows)
}

fn value_to_list_items(data: &Value) -> Vec<ListItem<'_>> {
    let mut items = Vec::new();

    match data {
        Value::Array(arr) => {
            for (i, value) in arr.iter().enumerate() {
                items.push(ListItem::new(format!("[{}] {}", i, value_to_string(value))));
            }
        }
        Value::Object(obj) => {
            for (key, value) in obj {
                items.push(ListItem::new(format!("{}: {}", key, value_to_string(value))));
            }
        }
        _ => {
            items.push(ListItem::new(value_to_string(data)));
        }
    }

    items
}

fn format_raw(value: &Value) -> String {
    serde_json::to_string_pretty(&value_to_json(value))
        .unwrap_or_else(|_| "Failed to serialize".to_string())
}

fn value_to_schema_text(value: &Value) -> Text<'_> {
    let mut lines = Vec::new();
    collect_schema(value, "", &mut lines);
    Text::from(lines)
}

fn collect_schema(value: &Value, prefix: &str, lines: &mut Vec<Line>) {
    match value {
        Value::Null => lines.push(Line::from(format!("{}null", prefix))),
        Value::Bool(_) => lines.push(Line::from(format!("{}boolean", prefix))),
        Value::Integer(_) => lines.push(Line::from(format!("{}integer", prefix))),
        Value::Float(_) => lines.push(Line::from(format!("{}float", prefix))),
        Value::String(s) => lines.push(Line::from(format!("{}string (len={})", prefix, s.len()))),
        Value::Array(arr) => {
            lines.push(Line::from(format!("{}array[{}] of:", prefix, arr.len())));
            if let Some(first) = arr.first() {
                collect_schema(first, "  ", lines);
            } else {
                lines.push(Line::from("  <empty>"));
            }
        }
        Value::Object(obj) => {
            lines.push(Line::from(format!("{}object with {} keys:", prefix, obj.len())));
            for (key, value) in obj {
                collect_schema(value, &format!("  {}: ", key), lines);
            }
        }
    }
}

fn value_to_string(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Integer(i) => i.to_string(),
        Value::Float(f) => f.to_string(),
        Value::String(s) => {
            if s.len() > 30 {
                format!("\"{}\"...", &s[..30])
            } else {
                format!("\"{}\"", s)
            }
        }
        Value::Array(arr) => format!("[{} items]", arr.len()),
        Value::Object(obj) => format!("{{{} keys}}", obj.len()),
    }
}

fn value_to_json(value: &Value) -> serde_json::Value {
    match value {
        Value::Null => serde_json::Value::Null,
        Value::Bool(b) => serde_json::Value::Bool(*b),
        Value::Integer(i) => serde_json::Value::Number(serde_json::Number::from(*i)),
        Value::Float(f) => serde_json::Value::Number(
            serde_json::Number::from_f64(*f).unwrap_or(serde_json::Number::from(0)),
        ),
        Value::String(s) => serde_json::Value::String(s.clone()),
        Value::Array(arr) => serde_json::Value::Array(arr.iter().map(|v| value_to_json(v)).collect()),
        Value::Object(obj) => serde_json::Value::Object(obj.iter().map(|(k, v)| (k.clone(), value_to_json(v))).collect()),
    }
}

/// Available themes
#[derive(Debug, Clone, Copy)]
pub enum Theme {
    Dark,
    Light,
    SolarizedDark,
    SolarizedLight,
    Dracula,
    Nord,
    Gruvbox,
}

impl Theme {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "light" => Theme::Light,
            "solarized-dark" | "solardark" => Theme::SolarizedDark,
            "solarized-light" | "solarball" => Theme::SolarizedLight,
            "dracula" => Theme::Dracula,
            "nord" => Theme::Nord,
            "gruvbox" => Theme::Gruvbox,
            _ => Theme::Dark,
        }
    }

    pub fn get_colors(&self) -> ThemeColors {
        use crate::ui::colors::*;
        match self {
            Theme::Dark => dark(),
            Theme::Light => light(),
            Theme::SolarizedDark => solarized_dark(),
            Theme::SolarizedLight => solarized_light(),
            Theme::Dracula => dracula(),
            Theme::Nord => nord(),
            Theme::Gruvbox => gruvbox(),
        }
    }
}

pub mod colors {
    use ratatui::style::Color;

    /// Theme color definitions
    #[derive(Clone, Copy)]
    #[allow(dead_code)]
    pub struct ThemeColors {
        pub background: Color,
        pub foreground: Color,
        pub border: Color,
        pub header: Color,
        pub accent: Color,
        pub warning: Color,
        pub error: Color,
        pub success: Color,
        pub selected: Color,
        pub dim: Color,
    }

    pub fn dark() -> ThemeColors {
        ThemeColors {
            background: Color::Black,
            foreground: Color::White,
            border: Color::Rgb(50, 50, 50),
            header: Color::Rgb(100, 140, 220),
            accent: Color::Rgb(80, 200, 120),
            warning: Color::Rgb(220, 160, 50),
            error: Color::Rgb(220, 70, 70),
            success: Color::Rgb(80, 200, 120),
            selected: Color::Rgb(45, 50, 70),
            dim: Color::Rgb(120, 120, 120),
        }
    }

    pub fn light() -> ThemeColors {
        ThemeColors {
            background: Color::White,
            foreground: Color::Black,
            border: Color::Rgb(200, 200, 200),
            header: Color::Rgb(60, 100, 200),
            accent: Color::Rgb(40, 160, 80),
            warning: Color::Rgb(200, 140, 30),
            error: Color::Rgb(200, 60, 60),
            success: Color::Rgb(40, 160, 80),
            selected: Color::Rgb(220, 230, 250),
            dim: Color::Rgb(130, 130, 130),
        }
    }

    pub fn solarized_dark() -> ThemeColors {
        ThemeColors {
            background: Color::Rgb(0, 43, 54),
            foreground: Color::Rgb(147, 161, 161),
            border: Color::Rgb(7, 54, 70),
            header: Color::Rgb(38, 139, 210),
            accent: Color::Rgb(42, 160, 110),
            warning: Color::Rgb(203, 75, 22),
            error: Color::Rgb(203, 75, 22),
            success: Color::Rgb(42, 160, 110),
            selected: Color::Rgb(15, 73, 96),
            dim: Color::Rgb(85, 110, 115),
        }
    }

    pub fn solarized_light() -> ThemeColors {
        ThemeColors {
            background: Color::Rgb(253, 246, 227),
            foreground: Color::Rgb(88, 110, 117),
            border: Color::Rgb(211, 228, 234),
            header: Color::Rgb(38, 139, 210),
            accent: Color::Rgb(42, 160, 110),
            warning: Color::Rgb(203, 75, 22),
            error: Color::Rgb(203, 75, 22),
            success: Color::Rgb(42, 160, 110),
            selected: Color::Rgb(220, 238, 242),
            dim: Color::Rgb(140, 160, 165),
        }
    }

    pub fn dracula() -> ThemeColors {
        ThemeColors {
            background: Color::Rgb(40, 42, 54),
            foreground: Color::Rgb(248, 248, 242),
            border: Color::Rgb(60, 63, 80),
            header: Color::Rgb(98, 114, 238),
            accent: Color::Rgb(80, 230, 130),
            warning: Color::Rgb(241, 214, 107),
            error: Color::Rgb(255, 85, 85),
            success: Color::Rgb(80, 230, 130),
            selected: Color::Rgb(68, 71, 92),
            dim: Color::Rgb(108, 113, 138),
        }
    }

    pub fn nord() -> ThemeColors {
        ThemeColors {
            background: Color::Rgb(29, 34, 46),
            foreground: Color::Rgb(203, 212, 226),
            border: Color::Rgb(50, 56, 72),
            header: Color::Rgb(129, 161, 193),
            accent: Color::Rgb(163, 190, 143),
            warning: Color::Rgb(219, 187, 145),
            error: Color::Rgb(191, 97, 106),
            success: Color::Rgb(163, 190, 143),
            selected: Color::Rgb(58, 65, 84),
            dim: Color::Rgb(95, 102, 119),
        }
    }

    pub fn gruvbox() -> ThemeColors {
        ThemeColors {
            background: Color::Rgb(29, 32, 33),
            foreground: Color::Rgb(232, 232, 232),
            border: Color::Rgb(70, 66, 58),
            header: Color::Rgb(79, 130, 183),
            accent: Color::Rgb(152, 196, 116),
            warning: Color::Rgb(214, 153, 61),
            error: Color::Rgb(204, 31, 39),
            success: Color::Rgb(152, 196, 116),
            selected: Color::Rgb(62, 58, 50),
            dim: Color::Rgb(124, 118, 108),
        }
    }
}
