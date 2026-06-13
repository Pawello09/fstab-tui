use crossterm::event::KeyCode;
use ratatui::{Frame, layout::{Constraint, Direction, Layout}, style::Modifier, widgets::{Block, Borders, List, ListState, Paragraph}};
use crate::{components::TextInput, fstab::fstab_entry::FSFile, screens::{screen::ScreenAction, tabs::ScreenTab}};

pub struct EditFSFileTab {
    type_list_state: ListState,
    text_input: TextInput,
    text_input_visible: bool
}

impl EditFSFileTab {
    const TYPE_LABELS: [&'static str; 3] = [
        "file",
        "none",
        "swap"
    ];

    pub fn new() -> Self {
        let mut type_list_state = ListState::default();
        type_list_state.select_first();
        let mut text_input = TextInput::new(&"".to_string(), 0, true);
        text_input.set_prefix(&"/".to_string());

        Self {
            type_list_state,
            text_input,
            text_input_visible: true
        }
    }

    pub fn init(&mut self, fs_file: FSFile) {
        self.text_input.set_value(&match fs_file {
            FSFile::Normal(ref value) => {
                if let Some(rest) = value.strip_prefix("/") {
                    rest.to_string()
                } else {
                    value.clone()
                }
            },
            _ => "".to_string()
        });
        self.text_input_visible = match fs_file {
            FSFile::Normal(_) => true,
            _ => false
        };
        self.type_list_state.select(Some(match fs_file {
            FSFile::Normal(_) => 0,
            FSFile::None => 1,
            FSFile::Swap => 2
        }));
    }

    pub fn get_fs_file(&self) -> FSFile {
        match self.type_list_state.selected() {
            Some(0) => FSFile::Normal(self.text_input.prefix.clone() + &self.text_input.get_value()),
            Some(2) => FSFile::Swap,
            _ => FSFile::None
        }
    }
}

impl ScreenTab for EditFSFileTab {
    fn render(&mut self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let chunks = Layout::default()
            .spacing(1)
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50)
            ]).split(area);

        let text_input_chunks = Layout::default()
            .spacing(1)
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3)
            ]).split(chunks[1]);

        let type_list = List::new(Self::TYPE_LABELS).highlight_style(Modifier::REVERSED);

        if self.text_input_visible {
            let text_input_block = Block::new()
                .borders(Borders::ALL)
                .title("path:");

            let text_input_block_inner = text_input_block.inner(text_input_chunks[0]);
            let text_input_cursor_x = text_input_block_inner.x + self.text_input.get_cursor_offset();
            let text_input_cursor_y = text_input_block_inner.y;

            frame.set_cursor_position((text_input_cursor_x, text_input_cursor_y));

            let text_input = Paragraph::new(self.text_input.get_input_text())
                .block(text_input_block);

            frame.render_widget(text_input, text_input_chunks[0]);
        }

        frame.render_stateful_widget(type_list, chunks[0], &mut self.type_list_state);
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> Option<crate::screens::screen::ScreenAction> {
        let screen_action = match key_event.code {
            KeyCode::Up => {
                self.type_list_state.select_previous();
                None
            },
            KeyCode::Down => {
                self.type_list_state.select_next();
                None
            },
            KeyCode::PageUp => {
                self.type_list_state.select_first();
                None
            },
            KeyCode::PageDown => {
                self.type_list_state.select_last();
                None
            },
            KeyCode::Left | KeyCode::Right | KeyCode::Home | KeyCode::End | KeyCode::Backspace | KeyCode::Delete | KeyCode::Char(_) => {
                if self.text_input_visible {
                    self.text_input.handle_key_event(key_event);
                    None
                } else {
                    match key_event.code {
                        KeyCode::Char('q') => Some(ScreenAction::NavigateTo(crate::app::Screen::Main)),
                        _ => None
                    }
                }
            },
            _ => None
        };

        self.text_input_visible = match self.type_list_state.selected() {
            Some(0) => true,
            _ => false
        };

        screen_action
    }
}
