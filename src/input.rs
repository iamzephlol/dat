use crossterm::event::{self, Event, KeyEvent};
use std::time::Duration;

/// Handles input events
pub struct InputHandler {}

impl InputHandler {
    pub fn new() -> Self {
        Self {}
    }

    /// Try to read an input event without blocking
    pub fn try_read(&mut self) -> Option<InputEvent> {
        if event::poll(Duration::from_millis(0)).ok()? {
            if let Ok(Event::Key(key)) = event::read() {
                return Some(InputEvent::Key(key));
            } else if let Ok(Event::Resize(w, h)) = event::read() {
                return Some(InputEvent::Resize(w as u16, h as u16));
            }
        }
        None
    }
}

/// Input events
#[derive(Debug, Clone)]
pub enum InputEvent {
    Key(KeyEvent),
    Resize(u16, u16),
}