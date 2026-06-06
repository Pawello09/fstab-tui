use crate::fstab::{Fstab, FstabLine};
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
            Span::from(" Up ").into(),
            Span::from("<Up>/<K>").style(Style::new().bold().blue()),
            Span::from(" Down ").into(),
            Span::from("<Down>/<J>").style(Style::new().bold().blue()),
            Span::from(" Quit ").into(),
            Span::from("<Q> ").style(Style::new().bold().blue())
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
            KeyCode::Esc | KeyCode::Char('q') => Some(ScreenAction::ExitApp),
            KeyCode::Up | KeyCode::Char('k') => {
                if let Some(selected) = self.fstab_table_state.selected() && selected > 0 {
                    let new_select = Some(selected-1);
                    self.fstab_table_state.select(new_select);
                    fstab.selected_line = self.fstab_table_state.selected();
                }
                None
            },
            KeyCode::Down | KeyCode::Char('j') => {
                let new_select = match self.fstab_table_state.selected() {
                    Some(selected) => selected + 1,
                    None => 0
                };

                if new_select < fstab.lines.len() {
                    self.fstab_table_state.select(Some(new_select));
                    fstab.selected_line = self.fstab_table_state.selected();
                }
                None
            },
            KeyCode::Char('e') => {
                match fstab.selected_line {
                    None => None,
                    Some(idx) => match fstab.lines[idx] {
                        FstabLine::Entry(_) => Some(ScreenAction::NavigateTo(crate::app::Screen::EntryEdit)),
                        FstabLine::Comment(_) => Some(ScreenAction::ShowPopup(crate::app::Popup::CommentEdit)),
                        _ => None
                    }
                }
            },
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

        fstab.selected_line = fstab_table_state.selected();

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
