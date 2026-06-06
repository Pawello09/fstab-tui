use crossterm::event::KeyEvent;
use ratatui::Frame;
use crate::fstab::Fstab;

pub enum ScreenAction {
    NavigateTo(crate::app::Screen),
    ShowPopup(crate::app::Popup),
    HidePopup,
    Exit
}

pub trait Screen {
    fn render(&mut self, frame: &mut Frame, area: ratatui::layout::Rect, app: &Fstab);
    fn handle_key_event(&mut self, key_event: KeyEvent, fstab: &mut Fstab) -> Option<ScreenAction>;
}
