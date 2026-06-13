use crate::fstab::{Fstab, FstabLine};
use crate::popups::comment_edit_popup::CommentEditPopupData;
use crate::popups::delete_line_popup::DeleteLinePopupData;
use crate::popups::new_line_popup::{LinePosition, NewLinePopupData};
use crate::screens::entry_edit_screen::EntryEditScreenData;
use crate::screens::screen::ScreenAction;
use super::screen::Screen;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::symbols::border;
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Padding, Row, Table, TableState};

pub struct MainScreen {
    pub fstab_table_state: TableState,
}

impl Screen for MainScreen {
    fn render(&mut self, frame: &mut Frame, area: ratatui::layout::Rect, fstab: &Fstab) {
        let title = Line::from(" fstab tui ").style(Style::new().bold()).alignment(Alignment::Center);
        let keybinds = Line::from(vec![
            Span::from(" up ").into(),
            Span::from("<up>").style(Style::new().bold().blue()),
            Span::from(" down ").into(),
            Span::from("<down>").style(Style::new().bold().blue()),
            Span::from(" first line ").into(),
            Span::from("<page up>").style(Style::new().bold().blue()),
            Span::from(" last line ").into(),
            Span::from("<page down>").style(Style::new().bold().blue()),
            Span::from(" move up ").into(),
            Span::from("<[>").style(Style::new().bold().blue()),
            Span::from(" move down ").into(),
            Span::from("<]>").style(Style::new().bold().blue()),
            Span::from(" quit ").into(),
            Span::from("<q>/<esc>").style(Style::new().bold().blue()),
            Span::from(" write ").into(),
            Span::from("<w>").style(Style::new().bold().blue()),
            Span::from(" new ").into(),
            Span::from("<o>/<O>").style(Style::new().bold().blue()),
            Span::from(" edit ").into(),
            Span::from("<e>").style(Style::new().bold().blue()),
            Span::from(" delete ").into(),
            Span::from("<d>").style(Style::new().bold().blue()),
            Span::from(" ")
        ]).alignment(Alignment::Center);
        let block = Block::bordered()
            .title(title)
            .title_bottom(keybinds)
            .border_set(border::THICK)
            .padding(Padding::new(2, 2, 1, 1));

        self.render_fstab_table(frame, block.inner(area), &fstab);
        frame.render_widget(block, area);
    }

    fn handle_key_event(&mut self, key_event: KeyEvent, fstab: &mut Fstab) -> Option<ScreenAction> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => Some(ScreenAction::ShowPopup(crate::app::Popup::Exit)),
            KeyCode::Up => {
                self.fstab_table_state.select_previous();
                None
            },
            KeyCode::Down => {
                self.fstab_table_state.select_next();
                None
            },
            KeyCode::PageUp => {
                self.fstab_table_state.select_first();
                None
            },
            KeyCode::PageDown => {
                self.fstab_table_state.select_last();
                None
            },
            KeyCode::Char('e') => {
                match self.fstab_table_state.selected() {
                    Some(idx) => match &fstab.lines[idx] {
                        FstabLine::Entry(entry) => Some(ScreenAction::NavigateTo(crate::app::Screen::EntryEdit(
                            EntryEditScreenData {
                                entry: entry.clone(),
                                fstab_line: self.fstab_table_state.selected()
                            }
                        ))),
                        FstabLine::Comment(_) => Some(ScreenAction::ShowPopup(crate::app::Popup::CommentEdit(
                            CommentEditPopupData {
                                fstab_line: Some(idx)
                            }
                        ))),
                        _ => None
                    },
                    _ => None
                }
            },
            KeyCode::Char('o') => Some(ScreenAction::ShowPopup(crate::app::Popup::NewLine(
                NewLinePopupData {
                    fstab_line: self.fstab_table_state.selected(),
                    new_line_position: LinePosition::Below
                }
            ))),
            KeyCode::Char('O') => Some(ScreenAction::ShowPopup(crate::app::Popup::NewLine(
                NewLinePopupData {
                    fstab_line: self.fstab_table_state.selected(),
                    new_line_position: LinePosition::Above
                }
            ))),
            KeyCode::Char('d') => {
                match self.fstab_table_state.selected() {
                    Some(idx) => Some(ScreenAction::ShowPopup(crate::app::Popup::DeleteLine(
                        DeleteLinePopupData {
                            fstab_line: Some(idx),
                        }
                    ))),
                    _ => None
                }
            },
            KeyCode::Char('[') => {
                match self.fstab_table_state.selected() {
                    Some(idx) => {
                        if idx > 0 {
                            fstab.lines.swap(idx, idx-1);
                            self.fstab_table_state.select_previous();
                        }
                    },
                    _ => {}
                }
                None
            },
            KeyCode::Char(']') => {
                match self.fstab_table_state.selected() {
                    Some(idx) => {
                        if idx + 1 < fstab.lines.len() {
                            fstab.lines.swap(idx, idx+1);
                            self.fstab_table_state.select_next();
                        }
                    },
                    _ => {}
                }
                None
            },
            KeyCode::Char('w') => Some(ScreenAction::ShowPopup(crate::app::Popup::Write)),
            _ => None
        }
    }
}

impl MainScreen {
    pub fn new(fstab: &mut Fstab) -> Self {
        let mut fstab_table_state = TableState::default();
        fstab_table_state.select(match fstab.lines.len() {
            0 => None,
            _ => Some(0)
        });

        Self {
            fstab_table_state
        }
    }

    fn render_fstab_table(&mut self, frame: &mut Frame, area: Rect, fstab: &Fstab) {
        let rows = fstab.lines.iter().enumerate().map(|(idx, fstab_line)|
            match fstab_line {
                FstabLine::Comment(_) => Row::new([
                    Text::from((idx+1).to_string()).alignment(Alignment::Right),
                    Text::from(fstab_line.to_string()).style(Style::new().bg(Color::Indexed(22)).fg(Color::White))
                ]),
                FstabLine::Entry(_) => Row::new([
                    Text::from((idx+1).to_string()).alignment(Alignment::Right),
                    Text::from(fstab_line.to_string()).style(Style::new().bg(Color::Indexed(27)).fg(Color::White))
                ]),
                _ => Row::new([
                    Text::from((idx+1).to_string()).alignment(Alignment::Right),
                    Text::from(fstab_line.to_string())
                ]),
            }
        );

        let widths = [
            Constraint::Length(3),
            Constraint::Fill(1)
        ];

        let table = Table::new(rows, widths)
            .column_spacing(1)
            .row_highlight_style(Modifier::REVERSED);

        frame.render_stateful_widget(table, area, &mut self.fstab_table_state);
    }

}
