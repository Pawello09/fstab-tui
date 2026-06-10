use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::{Constraint, Direction, Layout, Rect}, style::{Modifier, Style}, widgets::{Block, Borders, Clear, List, ListState, Padding, Paragraph}};
use crate::{fstab::{Fstab, FstabLine, fstab_entry::{FSFile, FSFreq, FSMntOp, FSPassNo, FSSpec, FSVFSType, FstabEntry}}, popups::popup::{Popup, PopupAction, get_centered_area}};

#[derive(Debug, Clone, Copy)]
pub enum LinePosition {
    Below,
    Above
}

#[derive(Debug, Clone, Copy)]
pub struct NewLinePopupData {
    pub new_line_position: LinePosition,
    pub fstab_line: Option<usize>
}

pub struct NewLinePopup {
    line_type_list_state: ListState,
    pub new_line_position: LinePosition,
    pub fstab_line: Option<usize>
}

impl Popup for NewLinePopup {
    fn handle_key_event(&mut self, key_event: KeyEvent, fstab: &mut Fstab) -> Option<PopupAction> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => Some(PopupAction::Close),
            KeyCode::Enter => {
                self.list_submit_action(fstab)
            },
            KeyCode::Up => {
                self.list_prev_line();
                None
            },
            KeyCode::Down => {
                self.list_next_line();
                None
            },
            _ => None
        }
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let popup_area = get_centered_area(26, 12, area);
        frame.render_widget(Clear, popup_area);
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" new line ")
            .padding(Padding::new(2, 2, 1, 1));

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Fill(1)
            ]).spacing(1)
            .split(block.inner(popup_area));

        frame.render_widget(block, popup_area);

        let title = Paragraph::new("choose line type:").style(Style::new().bold());
        let line_type_list = List::new(["empty", "comment", "entry", "cancel"])
            .highlight_style(Modifier::REVERSED);

        frame.render_widget(title, chunks[0]);
        frame.render_stateful_widget(line_type_list, chunks[1], &mut self.line_type_list_state);
    }
}

impl NewLinePopup {
    pub fn new() -> Self {
        let mut line_type_list_state = ListState::default();
        line_type_list_state.select_first();
        Self {
            line_type_list_state,
            new_line_position: LinePosition::Below,
            fstab_line: None
        }
    }

    pub fn init(&mut self, data: NewLinePopupData) {
        self.line_type_list_state.select_first();
        self.new_line_position = data.new_line_position;
        self.fstab_line = data.fstab_line;
    }

    fn list_prev_line(&mut self) {
        self.line_type_list_state.select_previous();
    }

    fn list_next_line(&mut self) {
        self.line_type_list_state.select_next();
    }

    fn list_submit_action(&mut self, fstab: &mut Fstab) -> Option<PopupAction> {
        if let Some(selected) = self.line_type_list_state.selected() {
            match selected {
                0 => self.add_line(FstabLine::EmptyLine, fstab),
                1 => self.add_line(FstabLine::Comment("".to_string()), fstab),
                2 => self.add_line(FstabLine::Entry(FstabEntry {
                    fs_spec: FSSpec::UUID("".to_string()),
                    fs_file: FSFile::Normal("/".to_string()),
                    fs_vfs: FSVFSType::Ext4,
                    fs_mntops: vec![FSMntOp::RW],
                    fs_freq: FSFreq::NoDump,
                    fs_passno: FSPassNo::CheckOther
                }), fstab),
                _ => Some(PopupAction::Close)
            }
        } else {
            None
        }
    }

    fn add_line(&mut self, fstab_line: FstabLine, fstab: &mut Fstab) -> Option<PopupAction> {
        let new_line_idx = match self.new_line_position {
            LinePosition::Below => match self.fstab_line {
                Some(line) => line + 1,
                None => 0
            },
            LinePosition::Above => match self.fstab_line {
                Some(line) => line,
                None => 0
            }
        };

        fstab.lines.insert(new_line_idx, fstab_line);

        match self.new_line_position {
            LinePosition::Below => Some(PopupAction::CloseAndMoveToNextLine),
            LinePosition::Above => Some(PopupAction::Close)
        }
    }
}
