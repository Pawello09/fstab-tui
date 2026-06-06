use crate::fstab::{Fstab, FstabLine};
use crate::screens::screen::ScreenAction;
use super::screen::Screen;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Text;
use ratatui::widgets::{Row, Table, TableState};

pub struct MainScreen {
    pub fstab_table_state: TableState,
}

impl Screen for MainScreen {
    fn render(&mut self, frame: &mut Frame, area: ratatui::layout::Rect, fstab: &Fstab) {
        self.render_fstab_table(frame, area, &fstab);
    }
    fn handle_key_event(&mut self, key_event: KeyEvent, fstab: &mut Fstab) -> Option<ScreenAction> {
        match key_event.code {
            KeyCode::Char('q') => Some(ScreenAction::Exit),
            KeyCode::Up => {
                if let Some(selected) = self.fstab_table_state.selected() && selected > 0 {
                    self.fstab_table_state.select(Some(selected-1));
                }
                None
            },
            KeyCode::Down => {
                let new_idx = match self.fstab_table_state.selected() {
                    Some(idx) => idx + 1,
                    None => 0
                };

                if new_idx < fstab.lines.len() {
                    self.fstab_table_state.select(Some(new_idx))
                }
                None
            }
            _ => None
        }
    }
}

impl MainScreen {
    pub fn new() -> Self {
        let mut fstab_table_state = TableState::default();
        fstab_table_state.select_first();
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
