use crossterm::event::KeyEvent;
use crate::app::App;
use crate::screens::screen::{Screen, ScreenAction};


pub fn handle_key_event(key_event: KeyEvent, app: &mut App) {
    let action = match app.current_screen {
        crate::app::Screen::Main => app.main_screen.handle_key_event(key_event, &mut app.fstab),
        _ => None
    };

    if let Some(action) = action {
        handle_screen_action(action, app);
    }
}

fn handle_screen_action(action: ScreenAction, app: &mut App) {
    match action {
        ScreenAction::NavigateTo(screen) => app.navigate_to(screen),
        ScreenAction::ShowPopup(popup) => app.show_popup(popup),
        ScreenAction::HidePopup => app.hide_popup(),
        ScreenAction::Exit => app.exit(),
        _ => {}
    }
}
