use crossterm::event::KeyCode;
use ratatui::{Frame, layout::{Constraint, Direction, Layout}, style::Modifier, widgets::{Block, Borders, List, ListState, Paragraph}};
use crate::{components::TextInput, fstab::fstab_entry::FSVFSType, screens::{screen::ScreenAction, tabs::ScreenTab}};

pub struct EditFSVFSTypeTab {
    type_list_state: ListState,
    text_input: TextInput,
    text_input_visible: bool
}

impl EditFSVFSTypeTab {
    const TYPE_LABELS: [&'static str; 5] = [
        "ext4",
        "xfs",
        "btrfs",
        "f2fs",
        "custom"
    ];

    pub fn new() -> Self {
        let mut type_list_state = ListState::default();
        type_list_state.select_first();
        let text_input = TextInput::new(&"".to_string(), 0);

        Self {
            type_list_state,
            text_input,
            text_input_visible: false
        }
    }

    pub fn init(&mut self, fs_vfs_type: FSVFSType) {
        self.text_input.set_value(&match fs_vfs_type {
            FSVFSType::Custom(ref value) => value.clone(),
            _ => "".to_string()
        });
        self.text_input_visible = match fs_vfs_type {
            FSVFSType::Custom(_) => true,
            _ => false
        };
        self.type_list_state.select(Some(match fs_vfs_type {
            FSVFSType::Ext4 => 0,
            FSVFSType::Xfs => 1,
            FSVFSType::Btrfs => 2,
            FSVFSType::F2fs => 3,
            FSVFSType::Custom(_) => 4
        }));
    }

    pub fn get_fs_vfs_type(&self) -> FSVFSType {
        match self.type_list_state.selected() {
            Some(0) => FSVFSType::Ext4,
            Some(1) => FSVFSType::Xfs,
            Some(2) => FSVFSType::Btrfs,
            Some(3) => FSVFSType::F2fs,
            _ => FSVFSType::Custom(self.text_input.value.clone())
        }
    }
}

impl ScreenTab for EditFSVFSTypeTab {
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
            let text_input_cursor_x = text_input_block_inner.x + self.text_input.cursor_position;
            let text_input_cursor_y = text_input_block_inner.y;

            frame.set_cursor_position((text_input_cursor_x, text_input_cursor_y));

            let text_input = Paragraph::new(self.text_input.value.clone())
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
            Some(4) => true,
            _ => false
        };

        screen_action
    }
}
