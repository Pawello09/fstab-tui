use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};
use crate::fstab::Fstab;

pub enum PopupAction {
    Close,
    ExitApp
}

pub trait Popup {
    fn handle_key_event(&mut self, key_event: KeyEvent, fstab: &mut Fstab) -> Option<PopupAction>;
    fn render(&self, frame: &mut Frame, area: Rect);
}

pub fn get_centered_area(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.width.saturating_sub(width) / 2;
    let y = area.height.saturating_sub(height) / 2;
    Rect::new(x, y, width, height)
}
