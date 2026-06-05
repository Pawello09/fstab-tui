use crate::fstab::Fstab;

pub enum CurrentScreen {
    Main,
    Editing,
    Exiting
}

pub struct App {
    pub fstab: Fstab,
    pub current_screen: CurrentScreen,
}

impl App {
   pub fn new(fstab_path: &str) -> App {
       App {
           fstab: Fstab::new(fstab_path),
           current_screen: CurrentScreen::Main
       }
   }
}
