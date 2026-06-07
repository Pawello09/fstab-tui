use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::{Constraint, Direction, Layout, Rect, Alignment}, style::{Color, Style}, widgets::{Block, Borders, Clear, Padding, Paragraph}};
use crate::{fstab::{Fstab}, popups::popup::{Popup, PopupAction, get_centered_area}};

#[derive(PartialEq)]
pub enum Selection {
    ExitButton,
    CancelButton
}

#[derive(Debug, Clone, Copy)]
pub struct ExitPopupData {
    pub fstab_line: Option<usize>
}

pub struct ExitPopup {
    selection: Selection,
}

impl Popup for ExitPopup {
    fn handle_key_event(&mut self, key_event: KeyEvent, fstab: &mut Fstab) -> Option<PopupAction> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => Some(PopupAction::Close),
            KeyCode::Enter => {
                match self.selection {
                    Selection::CancelButton => Some(PopupAction::Close),
                    Selection::ExitButton => {
                        Some(PopupAction::ExitApp)
                    },
                    _ => None
                }
            },
            KeyCode::Left => {
                match self.selection {
                    Selection::ExitButton => {
                        self.selection = Selection::CancelButton;
                    },
                    _ => {}
                };
                None
            },
            KeyCode::Right => {
                match self.selection {
                    Selection::CancelButton => {
                        self.selection = Selection::ExitButton;
                    },
                    _ => {}
                };
                None
            },
            _ => None
        }
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let popup_area = get_centered_area(60, 11, area);
        frame.render_widget(Clear, popup_area);
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" exit ")
            .padding(Padding::new(2, 2, 1, 1));

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(3),
            ]).spacing(1).split(block.inner(popup_area));

        frame.render_widget(block, popup_area);

        let title = Paragraph::new("are you sure you want to exit?").style(Style::new().bold()).centered();
        let warning = Paragraph::new("note: progress will not be saved").style(Style::new().red()).centered();

        frame.render_widget(title, chunks[0]);
        frame.render_widget(warning, chunks[1]);

        let button_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50)
            ]).spacing(3).split(chunks[2]);

        let cancel_style = if self.selection == Selection::CancelButton {
            Style::default().bg(Color::Gray).fg(Color::Black)
        } else {
            Style::default().bg(Color::DarkGray).fg(Color::White)
        };

        let cancel_button = Paragraph::new(" cancel ")
            .block(Block::default().borders(Borders::ALL))
            .style(cancel_style)
            .alignment(Alignment::Center);

        let exit_style = if self.selection == Selection::ExitButton {
            Style::default().bg(Color::LightRed).fg(Color::Black)
        } else {
            Style::default().bg(Color::Indexed(124)).fg(Color::White)
        };

        let exit_button = Paragraph::new(" exit ")
            .block(Block::default().borders(Borders::ALL))
            .style(exit_style)
            .alignment(Alignment::Center);


        frame.render_widget(cancel_button, button_chunks[0]);
        frame.render_widget(exit_button, button_chunks[1]);
    }
}

impl ExitPopup {
    pub fn new() -> Self {
        Self {
            selection: Selection::CancelButton,
        }
    }

    pub fn init(&mut self) {
        self.selection = Selection::CancelButton;
    }
}
