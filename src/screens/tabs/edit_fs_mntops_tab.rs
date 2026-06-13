use crossterm::event::KeyCode;
use ratatui::{Frame, layout::{Constraint, Direction, Layout}, style::Modifier, widgets::{Block, Borders, List, ListState, Paragraph}};
use crate::{components::TextInput, fstab::fstab_entry::FSMntOp, screens::{screen::ScreenAction, tabs::ScreenTab}};

pub struct EditFSMntOpsTab {
    options_list_state: ListState,
    options_selected: Vec<bool>,
    text_input: TextInput,
    text_input_visible: bool
}

impl EditFSMntOpsTab {
    const OPTION_LABELS: [&'static str; 10] = [
        "defaults",
        "rw",
        "ro",
        "auto",
        "noauto",
        "exec",
        "noexec",
        "sync",
        "async",
        "custom"
    ];

    pub fn new() -> Self {
        let mut options_list_state = ListState::default();
        options_list_state.select_first();

        let mut options_selected = vec![false; Self::OPTION_LABELS.len()];
        options_selected[0] = true;

        Self {
            options_list_state,
            options_selected,
            text_input: TextInput::default(),
            text_input_visible: false
        }
    }

    pub fn init(&mut self, fs_mntops: Vec<FSMntOp>) {
        self.options_list_state.select_first();

        self.options_selected = vec![false; Self::OPTION_LABELS.len()];

        self.text_input.set_value(&"".to_string());

        for fs_mntop in fs_mntops {
            self.options_selected[match fs_mntop {
                FSMntOp::Defaults => 0,
                FSMntOp::RW => 1,
                FSMntOp::RO => 2,
                FSMntOp::Auto => 3,
                FSMntOp::NoAuto => 4,
                FSMntOp::Exec => 5,
                FSMntOp::NoExec => 6,
                FSMntOp::Sync => 7,
                FSMntOp::Async => 8,
                FSMntOp::Custom(_) => 9
            }] = true;

            if let FSMntOp::Custom(value) = fs_mntop {
                if !self.text_input.value.is_empty() {
                    self.text_input.set_value(&(self.text_input.value.clone() + ","));
                }
                self.text_input.set_value(&(self.text_input.value.clone() + &value));
            }

            self.text_input_visible = self.options_selected.last().unwrap_or(&false).clone();
        }
    }

    pub fn get_fs_mntops(&self) -> Vec<FSMntOp> {
        let mut fs_mntops: Vec<FSMntOp> = vec![];

        self.options_selected.iter().enumerate().for_each(|(idx, selected)| {
            if *selected {
                fs_mntops.append(&mut match idx {
                    1 => vec![FSMntOp::RW],
                    2 => vec![FSMntOp::RO],
                    3 => vec![FSMntOp::Auto],
                    4 => vec![FSMntOp::NoAuto],
                    5 => vec![FSMntOp::Exec],
                    6 => vec![FSMntOp::NoExec],
                    7 => vec![FSMntOp::Sync],
                    8 => vec![FSMntOp::Async],
                    9 => {
                        self.text_input.value.split(',').map(|x| FSMntOp::Custom(x.to_string().clone())).collect()
                    },
                    _ => vec![FSMntOp::Defaults]
                });
            }
        });

        fs_mntops
    }
}

impl ScreenTab for EditFSMntOpsTab {
    fn render(&mut self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let chunks = Layout::default()
            .spacing(1)
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50)
            ]).split(area);

        let text_input_chunks = Layout::default()
            .spacing(1)
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3)
            ]).split(chunks[1]);

        if self.text_input_visible {
            let text_input_block = Block::new()
                .borders(Borders::ALL)
                .title("custom options:");

            let text_input_block_inner = text_input_block.inner(text_input_chunks[0]);
            let text_input_cursor_x = text_input_block_inner.x + self.text_input.get_cursor_offset();
            let text_input_cursor_y = text_input_block_inner.y;

            frame.set_cursor_position((text_input_cursor_x, text_input_cursor_y));

            let text_input = Paragraph::new(self.text_input.get_input_text())
                .block(text_input_block);

            frame.render_widget(text_input, text_input_chunks[0]);
        }
        let options_list = List::new(Self::OPTION_LABELS.iter().enumerate().map(|(idx, option_label)| {
            if self.options_selected[idx] {
                "[x] ".to_string() + &option_label
            } else {
                "[ ] ".to_string() + &option_label
            }
        }).collect::<Vec<_>>()).highlight_style(Modifier::REVERSED);

        frame.render_stateful_widget(options_list, chunks[0], &mut self.options_list_state);
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> Option<crate::screens::screen::ScreenAction> {
        let screen_action = match key_event.code {
            KeyCode::Up => {
                self.options_list_state.select_previous();
                None
            },
            KeyCode::Down => {
                self.options_list_state.select_next();
                None
            },
            KeyCode::PageUp => {
                self.options_list_state.select_first();
                None
            },
            KeyCode::PageDown => {
                self.options_list_state.select_last();
                None
            },
            KeyCode::Char(' ') => {
                if let Some(idx) = self.options_list_state.selected() {
                    self.options_selected[idx] = !self.options_selected[idx];
                }
                None
            },
            KeyCode::Left | KeyCode::Right | KeyCode::Home | KeyCode::End | KeyCode::Backspace | KeyCode::Delete | KeyCode::Char(_) => {
                if self.text_input_visible {
                    self.text_input.handle_key_event(key_event);
                    None
                } else {
                    match key_event.code {
                        KeyCode::Char('q') => Some(ScreenAction::NavigateTo(crate::app::Screen::Main)),
                        _ => None
                    }
                }
            },
            _ => None
        };

        self.text_input_visible = self.options_selected.last().unwrap_or(&false).clone();

        screen_action
    }
}

