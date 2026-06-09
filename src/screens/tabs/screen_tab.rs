use crossterm::event::KeyEvent;
use ratatui::Frame;
use crate::screens::screen::ScreenAction;

pub trait ScreenTab {
    fn render(&mut self, frame: &mut Frame, area: ratatui::layout::Rect);
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<ScreenAction>;
}

