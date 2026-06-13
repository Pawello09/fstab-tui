use crossterm::event::KeyCode;
use ratatui::{Frame, layout::{Constraint, Direction, Layout}, style::Modifier, widgets::{List, ListState}};
use crate::{fstab::fstab_entry::FSPassNo, screens::{screen::ScreenAction, tabs::ScreenTab}};

pub struct EditFSPassNoTab {
    type_list_state: ListState
}

impl EditFSPassNoTab {
    const TYPE_LABELS: [&'static str; 3] = [
        "no check",
        "check root",
        "check other"
    ];

    pub fn new() -> Self {
        let mut type_list_state = ListState::default();
        type_list_state.select_first();

        Self {
            type_list_state
        }
    }

    pub fn init(&mut self, fs_passno: FSPassNo) {
        self.type_list_state.select(Some(match fs_passno {
            FSPassNo::NoCheck => 0,
            FSPassNo::CheckRoot => 1,
            FSPassNo::CheckOther => 2
        }));
    }

    pub fn get_fs_passno(&self) -> FSPassNo {
        match self.type_list_state.selected() {
            Some(1) => FSPassNo::CheckRoot,
            Some(2) => FSPassNo::CheckOther,
            _ => FSPassNo::NoCheck
        }
    }
}

impl ScreenTab for EditFSPassNoTab {
    fn render(&mut self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let chunks = Layout::default()
            .spacing(1)
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50)
            ]).split(area);

        let type_list = List::new(Self::TYPE_LABELS).highlight_style(Modifier::REVERSED);

        frame.render_stateful_widget(type_list, chunks[0], &mut self.type_list_state);
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> Option<crate::screens::screen::ScreenAction> {
        match key_event.code {
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
                match key_event.code {
                    KeyCode::Char('q') => Some(ScreenAction::NavigateTo(crate::app::Screen::Main)),
                    _ => None
                }
            },
            _ => None
        }
    }
}

