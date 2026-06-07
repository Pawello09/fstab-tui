use crate::fstab::Fstab;
use crate::popups::delete_line_popup::{DeleteLinePopup, DeleteLinePopupData};
use crate::popups::exit_popup::ExitPopup;
use crate::popups::new_line_popup::{NewLinePopup, NewLinePopupData};
use crate::popups::write_error_popup::WriteErrorPopup;
use crate::popups::write_successful_popup::WriteSuccessfulPopup;
use crate::screens::entry_edit_screen::EntryEditScreen;
use crate::screens::main_screen::MainScreen;
use crate::popups::comment_edit_popup::{CommentEditPopup, CommentEditPopupData};
use crate::popups::write_popup::WritePopup;

pub enum Screen {
    CorruptedFile,
    MissingFile,
    Main,
    EntryEdit
}

pub enum Popup {
    CommentEdit(CommentEditPopupData),
    NewLine(NewLinePopupData),
    DeleteLine(DeleteLinePopupData),
    Write,
    WriteSuccessful,
    WriteError,
    Exit
}

pub struct App {
    pub fstab: Fstab,
    pub current_screen: Screen,
    pub current_popup: Option<Popup>,
    pub exited: bool,

    pub main_screen: MainScreen,
    pub entry_edit_screen: EntryEditScreen,

    pub comment_edit_popup: CommentEditPopup,
    pub new_line_popup: NewLinePopup,
    pub delete_line_popup: DeleteLinePopup,
    pub exit_popup: ExitPopup,
    pub write_popup: WritePopup,
    pub write_successful_popup: WriteSuccessfulPopup,
    pub write_error_popup: WriteErrorPopup
}

impl App {
   pub fn new(fstab_path: &str) -> App {
       let mut fstab = Fstab::new(fstab_path);
       let main_screen = MainScreen::new(&mut fstab);
       let entry_edit_screen = EntryEditScreen::new();
       App {
           fstab,
           current_screen: Screen::Main,
           current_popup: None,
           exited: false,
           main_screen,
           entry_edit_screen,
           comment_edit_popup: CommentEditPopup::new(),
           new_line_popup: NewLinePopup::new(),
           delete_line_popup: DeleteLinePopup::new(),
           exit_popup: ExitPopup::new(),
           write_popup: WritePopup::new(),
           write_successful_popup: WriteSuccessfulPopup::new(),
           write_error_popup: WriteErrorPopup::new()
       }
   }

   pub fn navigate_to(&mut self, screen: Screen) {
       self.current_screen = screen;
   }

   pub fn show_popup(&mut self, popup: Popup) {
       match popup {
           Popup::CommentEdit(data) => self.comment_edit_popup.init(data.clone(), &self.fstab),
           Popup::NewLine(data) => self.new_line_popup.init(data.clone()),
           Popup::DeleteLine(data) => self.delete_line_popup.init(data.clone()),
           Popup::Exit => self.exit_popup.init(),
           Popup::Write => self.write_popup.init(&self.fstab),
           Popup::WriteSuccessful => self.write_successful_popup.init(&self.fstab),
           Popup::WriteError => self.write_error_popup.init(&self.fstab)
       };
       self.current_popup = Some(popup);
   }

   pub fn hide_popup(&mut self) {
       self.current_popup = None;
   }

   pub fn exit(&mut self) {
       self.exited = true;
   }
}
