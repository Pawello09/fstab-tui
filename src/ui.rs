use super::app::App;
use ratatui::Frame;
use crate::screens::screen::Screen;
use crate::popups::popup::Popup;

pub fn render(app: &mut App, frame: &mut Frame) {
    match app.current_screen {
        super::app::Screen::Main => app.main_screen.render(frame, frame.area(), &app.fstab),
        super::app::Screen::EntryEdit => app.entry_edit_screen.render(frame, frame.area(), &app.fstab),
        _ => {}
    };
    if let Some(popup) = &app.current_popup {
        match popup {
            super::app::Popup::CommentEdit => app.comment_edit_popup.render(frame, frame.area()),
            _ => {}
        }
    }
}
