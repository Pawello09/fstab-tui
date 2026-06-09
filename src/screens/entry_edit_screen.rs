use crate::fstab::fstab_entry::{FSSpec, FstabEntry};
use crate::fstab::{Fstab, FstabLine};
use crate::screens::screen::ScreenAction;
use crate::screens::tabs::EditFSFileTab;
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
    fs_spec_tab: EditFSSpecTab,
    fs_file_tab: EditFSFileTab,
    fstab_line: Option<usize>
}

impl Screen for EntryEditScreen {
    fn render(&mut self, frame: &mut Frame, area: ratatui::layout::Rect, _fstab: &Fstab) {
        let title = Line::from(" fstab tui - edit ").style(Style::new().bold()).alignment(Alignment::Center);
        let keybinds = Line::from(vec![]).alignment(Alignment::Center);
        let block = Block::bordered()
            .title(title)
            .title_bottom(keybinds)
            .border_set(border::THICK)
            .padding(Padding::new(2, 2, 1, 1));

        match self.active_tab {
            Tab::FSSpec => self.fs_spec_tab.render(frame, block.inner(area)),
            Tab::FSFile => self.fs_file_tab.render(frame, block.inner(area)),
            _ => {}
        }

        frame.render_widget(block, area);
    }

    fn handle_key_event(&mut self, key_event: KeyEvent, fstab: &mut Fstab) -> Option<ScreenAction> {
        match key_event.code {
            KeyCode::Esc => Some(ScreenAction::NavigateTo(crate::app::Screen::Main)),
            KeyCode::Enter => self.save(fstab),
            KeyCode::Char('[') => {
                self.prev_tab();
                None
            },
            KeyCode::Char(']') => {
                self.next_tab();
                None
            },
            _ => match self.active_tab {
                Tab::FSSpec => self.fs_spec_tab.handle_key_event(key_event),
                Tab::FSFile => self.fs_file_tab.handle_key_event(key_event),
                _ => None
            }
        }
    }
}

impl EntryEditScreen {
    pub fn new() -> Self {
        Self {
            active_tab: Tab::FSSpec,
            fs_spec_tab: EditFSSpecTab::new(),
            fs_file_tab: EditFSFileTab::new(),
            fstab_line: None
        }
    }

    pub fn init(&mut self, data: EntryEditScreenData) {
        self.active_tab = Tab::FSSpec;
        self.fstab_line = data.fstab_line;

        self.fs_spec_tab.init(data.entry.fs_spec);
        self.fs_file_tab.init(data.entry.fs_file);
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

    fn save(&self, fstab: &mut Fstab) -> Option<ScreenAction> {
        if let Some(line) = self.fstab_line {
            fstab.lines[line] = FstabLine::Entry(FstabEntry {
                fs_spec: self.fs_spec_tab.get_fs_spec(),
                fs_file: self.fs_file_tab.get_fs_file(),
                fs_vfs: crate::fstab::fstab_entry::FSVFSType::Ext4,
                fs_mntops: vec![],
                fs_freq: crate::fstab::fstab_entry::FSFreq::NoDump,
                fs_passno: crate::fstab::fstab_entry::FSPassNo::NoCheck
            });
            Some(ScreenAction::NavigateTo(crate::app::Screen::Main))
        } else {
            None
        }
    }
}
