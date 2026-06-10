use crate::fstab::fstab_entry::{FstabEntry};
use crate::fstab::{Fstab, FstabLine};
use crate::screens::screen::ScreenAction;
use crate::screens::tabs::{EditFSFileTab, EditFSFreqTab, EditFSPassNoTab, EditFSVFSTypeTab};
use super::screen::Screen;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Spacing};
use ratatui::style::{Color, Style};
use ratatui::symbols::border;
use ratatui::symbols::merge::MergeStrategy;
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders, Padding, Paragraph};
use super::tabs::{ScreenTab, EditFSSpecTab};

#[derive(PartialEq, Eq, Clone)]
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
    fstab_line: Option<usize>,
    fs_spec_tab: EditFSSpecTab,
    fs_file_tab: EditFSFileTab,
    fs_vfs_type_tab: EditFSVFSTypeTab,
    fs_freq_tab: EditFSFreqTab,
    fs_passno_tab: EditFSPassNoTab
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

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .spacing(1)
            .constraints([
                Constraint::Length(3),
                Constraint::Fill(1)
            ]).split(block.inner(area));

        let navbar_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .spacing(Spacing::Overlap(1))
            .constraints(Self::NAVBAR_LABELS.iter().map(|_| Constraint::Fill(1)))
            .split(chunks[0]);

        for (idx, navbar_label_text) in Self::NAVBAR_LABELS.iter().enumerate() {
            let navbar_label_block = Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .merge_borders(MergeStrategy::Exact)
                .border_style(Style::default().fg(Color::White));

            let navbar_label_selected = Some(self.active_tab.clone()) == match idx {
                0 => Some(Tab::FSSpec),
                1 => Some(Tab::FSFile),
                2 => Some(Tab::FSVFSType),
                3 => Some(Tab::FSMntOps),
                4 => Some(Tab::FSFreq),
                5 => Some(Tab::FSPassNo),
                _ => None
            };

            let navbar_label_style = if navbar_label_selected {
                Style::new()
                    .fg(Color::LightBlue)
                    .bold()
            } else {
                Style::new()
                    .fg(Color::White)
            };

            let navbar_label = Paragraph::new(*navbar_label_text)
                .block(navbar_label_block)
                .style(navbar_label_style)
                .centered();

            frame.render_widget(navbar_label, navbar_chunks[idx]);
        }

        match self.active_tab {
            Tab::FSSpec => self.fs_spec_tab.render(frame, chunks[1]),
            Tab::FSFile => self.fs_file_tab.render(frame, chunks[1]),
            Tab::FSVFSType => self.fs_vfs_type_tab.render(frame, chunks[1]),
            Tab::FSFreq => self.fs_freq_tab.render(frame, chunks[1]),
            Tab::FSPassNo => self.fs_passno_tab.render(frame, chunks[1]),
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
                Tab::FSVFSType => self.fs_vfs_type_tab.handle_key_event(key_event),
                Tab::FSFreq => self.fs_freq_tab.handle_key_event(key_event),
                Tab::FSPassNo => self.fs_passno_tab.handle_key_event(key_event),
                _ => None
            }
        }
    }
}

impl EntryEditScreen {
    const NAVBAR_LABELS: [&'static str; 6] = [
        "fs_spec",
        "fs_file",
        "fs_vfs_type",
        "fs_mntops",
        "fs_freq",
        "fs_passno"
    ];

    pub fn new() -> Self {
        Self {
            active_tab: Tab::FSSpec,
            fs_spec_tab: EditFSSpecTab::new(),
            fs_file_tab: EditFSFileTab::new(),
            fs_vfs_type_tab: EditFSVFSTypeTab::new(),
            fs_freq_tab: EditFSFreqTab::new(),
            fs_passno_tab: EditFSPassNoTab::new(),
            fstab_line: None
        }
    }

    pub fn init(&mut self, data: EntryEditScreenData) {
        self.active_tab = Tab::FSSpec;
        self.fstab_line = data.fstab_line;

        self.fs_spec_tab.init(data.entry.fs_spec);
        self.fs_file_tab.init(data.entry.fs_file);
        self.fs_vfs_type_tab.init(data.entry.fs_vfs);
        self.fs_freq_tab.init(data.entry.fs_freq);
        self.fs_passno_tab.init(data.entry.fs_passno);
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
                fs_vfs: self.fs_vfs_type_tab.get_fs_vfs_type(),
                fs_mntops: vec![],
                fs_freq: self.fs_freq_tab.get_fs_freq(),
                fs_passno: self.fs_passno_tab.get_fs_passno()
            });
            Some(ScreenAction::NavigateTo(crate::app::Screen::Main))
        } else {
            None
        }
    }
}
