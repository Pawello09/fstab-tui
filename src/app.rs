use ratatui::widgets::TableState;

use crate::fstab::Fstab;
use crate::screens::main_screen::MainScreen;

pub enum Screen {
    CorruptedFile,
    MissingFile,
    Main,
    Edit
}

pub enum Popup {
    Save,
    Exit
}

pub struct App {
    pub fstab: Fstab,
    pub current_screen: Screen,
    pub current_popup: Option<Popup>,
    pub exited: bool,
    pub main_screen: MainScreen,
}

impl App {
   pub fn new(fstab_path: &str) -> App {
       let fstab = Fstab::new(fstab_path);
       let main_screen = MainScreen::new();
       App {
           fstab,
           current_screen: Screen::Main,
           current_popup: None,
           exited: false,
           main_screen
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
