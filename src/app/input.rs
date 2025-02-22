/// Available modes for the application
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    /// Normal mode - for navigation
    Normal,
    /// Insert mode - for editing queries
    Insert,
}

/// Handles input state for the application
#[derive(Debug)]
pub struct InputHandler {
    /// Current input buffer
    buffer: String,
    /// Cursor position in the buffer
    cursor_position: usize,
    /// Current mode
    mode: Mode,
}

impl Default for InputHandler {
    fn default() -> Self {
        Self {
            buffer: String::new(),
            cursor_position: 0,
            mode: Mode::Normal,
        }
    }
}

impl InputHandler {
    /// Creates a new input handler
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets the current buffer content
    pub fn buffer(&self) -> &str {
        &self.buffer
    }

    /// Gets the current cursor position
    pub fn cursor_position(&self) -> usize {
        self.cursor_position
    }

    /// Gets the current mode
    pub fn mode(&self) -> Mode {
        self.mode
    }

    /// Toggles between Normal and Insert mode
    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            Mode::Normal => Mode::Insert,
            Mode::Insert => Mode::Normal,
        };
    }

    /// Inserts a character at the current cursor position
    pub fn insert_char(&mut self, c: char) {
        self.buffer.insert(self.cursor_position, c);
        self.cursor_position += 1;
    }

    /// Deletes the character before the cursor
    pub fn delete_char(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.buffer.remove(self.cursor_position);
        }
    }

    /// Moves the cursor left
    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    /// Moves the cursor right
    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.buffer.len() {
            self.cursor_position += 1;
        }
    }

    /// Clears the input buffer
    pub fn clear_buffer(&mut self) {
        self.buffer.clear();
        self.cursor_position = 0;
    }

    /// Returns and clears the buffer
    pub fn take_buffer(&mut self) -> String {
        let buffer = std::mem::take(&mut self.buffer);
        self.cursor_position = 0;
        buffer
    }
}