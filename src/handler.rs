use crossterm::event::KeyEvent;
use crate::app::App;
use crate::screens::screen::{Screen, ScreenAction};
use crate::popups::popup::{Popup, PopupAction};

pub fn handle_key_event(key_event: KeyEvent, app: &mut App) {
    if let Some(popup) = &app.current_popup {
        let popup_action = match popup {
            crate::app::Popup::CommentEdit => app.comment_edit_popup.handle_key_event(key_event, &mut app.fstab),
            _ => None
        };

        if let Some(popup_action) = popup_action {
            handle_popup_action(popup_action, app);
        }

        return;
    }

    let screen_action = match app.current_screen {
        crate::app::Screen::Main => app.main_screen.handle_key_event(key_event, &mut app.fstab),
        crate::app::Screen::EntryEdit => app.entry_edit_screen.handle_key_event(key_event, &mut app.fstab),
        _ => None
    };

    if let Some(screen_action) = screen_action {
        handle_screen_action(screen_action, app);
    }
}

fn handle_screen_action(action: ScreenAction, app: &mut App) {
    match action {
        ScreenAction::NavigateTo(screen) => app.navigate_to(screen),
        ScreenAction::ShowPopup(popup) => app.show_popup(popup),
        ScreenAction::HidePopup => app.hide_popup(),
        ScreenAction::ExitApp => app.exit(),
        _ => {}
    }
}

fn handle_popup_action(action: PopupAction, app: &mut App) {
    match action {
        PopupAction::Close => app.hide_popup(),
        PopupAction::ExitApp => app.exit(),
        _ => {}
    }
}
