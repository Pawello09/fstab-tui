use crossterm::event::KeyCode;
use ratatui::{Frame, layout::{Constraint, Direction, Layout}, style::Modifier, widgets::{Block, Borders, List, ListState, Paragraph}};
use crate::{components::TextInput, fstab::fstab_entry::FSSpec, screens::tabs::ScreenTab};

pub struct EditFSSpecTab {
    type_list_state: ListState,
    text_input: TextInput
}

impl EditFSSpecTab {
    const TYPE_LABELS: [&'static str; 5] = [
        "LABEL", "UUID", "PARTLABEL", "PARTUUID", "custom"
    ];

    pub fn new() -> Self {
        let mut type_list_state = ListState::default();
        type_list_state.select_first();
        let mut text_input = TextInput::default();
        text_input.set_prefix(&"LABEL=".to_string());

        Self {
            type_list_state,
            text_input
        }
    }

    pub fn init(&mut self, fs_spec: FSSpec) {
        self.text_input.set_value(&match fs_spec {
            FSSpec::Label(ref label) => label.to_string(),
            FSSpec::UUID(ref uuid) => uuid.to_string(),
            FSSpec::PartLabel(ref partlabel) => partlabel.to_string(),
            FSSpec::PartUUID(ref partuuid) => partuuid.to_string(),
            FSSpec::Custom(ref custom) => custom.to_string()
        });
        self.type_list_state.select(Some(match fs_spec {
            FSSpec::Label(_) => 0,
            FSSpec::UUID(_) => 1,
            FSSpec::PartLabel(_) => 2,
            FSSpec::PartUUID(_) => 3,
            FSSpec::Custom(_) => 4
        }));
        self.text_input.set_prefix(&match fs_spec {
            FSSpec::Label(_) => "LABEL=".to_string(),
            FSSpec::UUID(_) => "UUID=".to_string(),
            FSSpec::PartLabel(_) => "PARTLABEL=".to_string(),
            FSSpec::PartUUID(_) => "PARTUUID=".to_string(),
            _ => "".to_string()
        });
    }

    pub fn get_fs_spec(&self) -> FSSpec {
        let value = self.text_input.value.clone();
        match self.type_list_state.selected() {
            Some(0) => FSSpec::Label(value),
            Some(1) => FSSpec::UUID(value),
            Some(2) => FSSpec::PartLabel(value),
            Some(3) => FSSpec::PartUUID(value),
            _ => FSSpec::Custom(value)
        }
    }
}

impl ScreenTab for EditFSSpecTab {
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

        let text_input_title_text = match self.type_list_state.selected() {
            Some(idx) => match idx {
                0 => "label:",
                1 => "uuid:",
                2 => "partlabel:",
                3 => "partuuid:",
                4 => "custom value:",
                _ => "value:"
            }
            None => "value:"
        };

        let text_input_block = Block::new()
            .borders(Borders::ALL)
            .title(text_input_title_text);

        let text_input_block_inner = text_input_block.inner(text_input_chunks[0]);
        let text_input_cursor_x = text_input_block_inner.x + self.text_input.get_cursor_offset();
        let text_input_cursor_y = text_input_block_inner.y;

        frame.set_cursor_position((text_input_cursor_x, text_input_cursor_y));

        let text_input = Paragraph::new(self.text_input.get_input_text())
            .block(text_input_block);

        frame.render_stateful_widget(type_list, chunks[0], &mut self.type_list_state);
        frame.render_widget(text_input, text_input_chunks[0]);
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
                self.text_input.handle_key_event(key_event);
                None
            },
            _ => None
        };

        self.text_input.set_prefix(&match self.type_list_state.selected() {
            Some(0) => "LABEL=".to_string(),
            Some(1) => "UUID=".to_string(),
            Some(2) => "PARTLABEL=".to_string(),
            Some(3) => "PARTUUID=".to_string(),
            _ => "".to_string()
        });

        screen_action
    }
}
