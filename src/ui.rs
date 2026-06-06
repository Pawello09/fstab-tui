use super::app::App;
use ratatui::Frame;
use crate::screens::screen::Screen;

pub fn render(app: &mut App, frame: &mut Frame) {
    match app.current_screen {
       super::app::Screen::Main => app.main_screen.render(frame, frame.area(), &app.fstab),
       _ => {}
    }
}
