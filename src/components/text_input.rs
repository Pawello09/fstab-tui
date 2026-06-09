use crossterm::event::{KeyCode, KeyEvent};

pub struct TextInput {
    pub cursor_position: u16,
    pub value: String
}

impl TextInput {
    pub fn new(value: &String, cursor_position: u16) -> Self {
        Self {
            value: value.clone(),
            cursor_position: std::cmp::min(cursor_position, value.len() as u16)
        }
    }

    pub fn default() -> Self {
        Self {
            value: "".to_string(),
            cursor_position: 0
        }
    }

    pub fn backspace(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.value.remove(self.cursor_position as usize);
        }
    }

    pub fn delete(&mut self) {
        if (self.cursor_position as usize) < self.value.len() {
            self.value.remove(self.cursor_position as usize);
        }
    }

    pub fn write(&mut self, c: &char) {
        self.value.insert(self.cursor_position as usize, c.clone());
        self.cursor_position += 1;
    }

    pub fn left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position = self.cursor_position - 1;
        }
    }

    pub fn right(&mut self) {
        if (self.cursor_position as usize) < self.value.len() {
            self.cursor_position = self.cursor_position + 1;
        }
    }

    pub fn home(&mut self) {
        self.cursor_position = 0;
    }

    pub fn end(&mut self) {
        self.cursor_position = self.value.len() as u16;
    }

    pub fn set_value(&mut self, value: &String) {
        self.value = value.clone();
        self.cursor_position = self.value.len() as u16;
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Left => {
                self.left();
            },
            KeyCode::Right => {
                self.right();
            },
            KeyCode::Home => {
                self.home();
            },
            KeyCode::End => {
                self.end();
            },
            KeyCode::Backspace => {
                self.backspace();
            },
            KeyCode::Delete => {
                self.delete();
            },
            KeyCode::Char(c) => {
                self.write(&c);
            }
            _ => {}
        }
    }
}
