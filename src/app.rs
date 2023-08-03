use std::error;
use tui_input::Input;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub enum InputMode {
    Normal,
    Editing,
}

/// Application.
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,
     /// Current value of the input box
    pub input: Input,
    /// Current input mode
    pub input_mode: InputMode,
    /// History of recorded messages
    pub messages: Vec<String>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            input: Input::default(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            running: true,
            counter: 0,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
