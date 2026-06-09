use crate::fstab::fstab_entry::{FSFile, FSVFSType, FstabEntry};
use crate::fstab::{Fstab};
use crate::screens::screen::ScreenAction;
use super::screen::Screen;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::layout::Alignment;
use ratatui::style::Style;
use ratatui::symbols::border;
use ratatui::text::Line;
use ratatui::widgets::{Block, Padding};
use super::tabs::{ScreenTab, EditFSSpecTab};

pub enum Tab {
   FSSpec,
   FSFile,
   FSVFSType,
   FSMntOps,
   FSFreq,
   FSPassNo
}

#[derive(Debug, Clone)]
pub struct EntryEditScreenData {
    pub entry: FstabEntry,
    pub fstab_line: Option<usize>
}

pub struct EntryEditScreen {
    active_tab: Tab,
    fs_spec_tab: EditFSSpecTab
}

impl Screen for EntryEditScreen {
    fn render(&mut self, frame: &mut Frame, area: ratatui::layout::Rect, fstab: &Fstab) {
        let title = Line::from(" fstab tui - edit ").style(Style::new().bold()).alignment(Alignment::Center);
        let keybinds = Line::from(vec![]).alignment(Alignment::Center);
        let block = Block::bordered()
            .title(title)
            .title_bottom(keybinds)
            .border_set(border::THICK)
            .padding(Padding::new(2, 2, 1, 1));

        match self.active_tab {
            Tab::FSSpec => self.fs_spec_tab.render(frame, block.inner(area)),
            _ => {}
        }

        frame.render_widget(block, area);
    }

    fn handle_key_event(&mut self, key_event: KeyEvent, fstab: &mut Fstab) -> Option<ScreenAction> {
        match key_event.code {
            KeyCode::Esc => Some(ScreenAction::NavigateTo(crate::app::Screen::Main)),
            KeyCode::Char('[') => {
                self.prev_tab();
                None
            },
            KeyCode::Char(']') => {
                self.next_tab();
                None
            },
            _ => match self.active_tab {
                Tab::FSSpec => self.fs_spec_tab.handle_key_event(key_event, fstab),
                _ => None
            }
        }
    }
}

impl EntryEditScreen {
    pub fn new() -> Self {
        Self {
            active_tab: Tab::FSSpec,
            fs_spec_tab: EditFSSpecTab::new()
        }
    }

    pub fn init(&mut self, data: EntryEditScreenData) {
        self.active_tab = Tab::FSSpec;

        self.fs_spec_tab.init(data.entry.fs_spec);
    }

    fn next_tab(&mut self) {
        let new_tab = match self.active_tab {
            Tab::FSSpec => Some(Tab::FSFile),
            Tab::FSFile => Some(Tab::FSVFSType),
            Tab::FSVFSType => Some(Tab::FSMntOps),
            Tab::FSMntOps => Some(Tab::FSFreq),
            Tab::FSFreq => Some(Tab::FSPassNo),
            _ => None
        };

        if let Some(new_tab) = new_tab {
            self.active_tab = new_tab;
        }
    }

    fn prev_tab(&mut self) {
        let new_tab = match self.active_tab {
            Tab::FSPassNo => Some(Tab::FSFreq),
            Tab::FSFreq => Some(Tab::FSMntOps),
            Tab::FSMntOps => Some(Tab::FSVFSType),
            Tab::FSVFSType => Some(Tab::FSFile),
            Tab::FSFile => Some(Tab::FSSpec),
            _ => None
        };

        if let Some(new_tab) = new_tab {
            self.active_tab = new_tab;
        }
    }
}
