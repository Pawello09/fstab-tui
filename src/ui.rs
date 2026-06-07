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
            super::app::Popup::CommentEdit(_) => app.comment_edit_popup.render(frame, frame.area()),
            super::app::Popup::NewLine(_) => app.new_line_popup.render(frame, frame.area()),
            super::app::Popup::DeleteLine(_) => app.delete_line_popup.render(frame, frame.area()),
            super::app::Popup::Exit => app.exit_popup.render(frame, frame.area()),
            super::app::Popup::Write => app.write_popup.render(frame, frame.area()),
            super::app::Popup::WriteSuccessful => app.write_successful_popup.render(frame, frame.area()),
            super::app::Popup::WriteError => app.write_error_popup.render(frame, frame.area())
        }
    }
}
