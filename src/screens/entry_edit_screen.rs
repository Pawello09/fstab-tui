use crate::fstab::{Fstab};
use crate::screens::screen::ScreenAction;
use super::screen::Screen;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::layout::{Alignment};
use ratatui::style::{Style};
use ratatui::symbols::border;
use ratatui::text::{Line};
use ratatui::widgets::{Block, Padding};

pub struct EntryEditScreen {}

impl Screen for EntryEditScreen {
    fn render(&mut self, frame: &mut Frame, area: ratatui::layout::Rect, fstab: &Fstab) {
        let title = Line::from(" fstab tui - edit ").style(Style::new().bold()).alignment(Alignment::Center);
        let keybinds = Line::from(vec![]).alignment(Alignment::Center);
        let block = Block::bordered()
            .title(title)
            .title_bottom(keybinds)
            .border_set(border::THICK)
            .padding(Padding::new(2, 2, 1, 1));

        frame.render_widget(block, area);
    }

    fn handle_key_event(&mut self, key_event: KeyEvent, fstab: &mut Fstab) -> Option<ScreenAction> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => Some(ScreenAction::NavigateTo(crate::app::Screen::Main)),
            _ => None
        }
    }
}

impl EntryEditScreen {
    pub fn new() -> Self {
        Self {}
    }
}
