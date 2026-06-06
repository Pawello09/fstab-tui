use crate::fstab::Fstab;
use crate::screens::entry_edit_screen::EntryEditScreen;
use crate::screens::main_screen::MainScreen;
use crate::popups::comment_edit_popup::CommentEditPopup;

pub enum Screen {
    CorruptedFile,
    MissingFile,
    Main,
    EntryEdit
}

pub enum Popup {
    CommentEdit,
    Save,
    Exit
}

pub struct App {
    pub fstab: Fstab,
    pub current_screen: Screen,
    pub current_popup: Option<Popup>,
    pub exited: bool,

    pub main_screen: MainScreen,
    pub entry_edit_screen: EntryEditScreen,

    pub comment_edit_popup: CommentEditPopup
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
           comment_edit_popup: CommentEditPopup::new()
       }
   }

   pub fn navigate_to(&mut self, screen: Screen) {
       self.current_screen = screen;
   }

   pub fn show_popup(&mut self, popup: Popup) {
       self.current_popup = Some(popup);
   }

   pub fn hide_popup(&mut self) {
       self.current_popup = None;
   }

   pub fn exit(&mut self) {
       self.exited = true;
   }
}
