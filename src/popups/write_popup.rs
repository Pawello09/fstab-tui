use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::{Constraint, Direction, Layout, Rect, Alignment}, style::{Color, Style}, widgets::{Block, Borders, Clear, Padding, Paragraph}};
use crate::{fstab::Fstab, popups::popup::{Popup, PopupAction, get_centered_area}, screens::screen::ScreenAction};

#[derive(PartialEq)]
pub enum Selection {
    WriteButton,
    CancelButton
}

pub struct WritePopup {
    selection: Selection,
    path: String
}

impl Popup for WritePopup {
    fn handle_key_event(&mut self, key_event: KeyEvent, fstab: &mut Fstab) -> Option<PopupAction> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => Some(PopupAction::Close),
            KeyCode::Enter => {
                match self.selection {
                    Selection::CancelButton => Some(PopupAction::Close),
                    Selection::WriteButton => {
                        self.write_fstab_to_file(fstab)
                    },
                }
            },
            KeyCode::Left => {
                match self.selection {
                    Selection::WriteButton => {
                        self.selection = Selection::CancelButton;
                    },
                    _ => {}
                };
                None
            },
            KeyCode::Right => {
                match self.selection {
                    Selection::CancelButton => {
                        self.selection = Selection::WriteButton;
                    },
                    _ => {}
                };
                None
            },
            _ => None
        }
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let popup_area = get_centered_area(60, 13, area);
        frame.render_widget(Clear, popup_area);
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" write ")
            .padding(Padding::new(2, 2, 1, 1));

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(3),
            ]).spacing(1).split(block.inner(popup_area));

        frame.render_widget(block, popup_area);

        let title = Paragraph::new("are you sure you want to write this config to file?").style(Style::new().bold()).centered();
        let file_path = Paragraph::new("path: ".to_owned() + self.path.as_str()).style(Style::new().green()).centered();
        let warning = Paragraph::new("note: this cannot be undone").style(Style::new().red()).centered();

        frame.render_widget(title, chunks[0]);
        frame.render_widget(file_path, chunks[1]);
        frame.render_widget(warning, chunks[2]);

        let button_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50)
            ]).spacing(3).split(chunks[3]);

        let cancel_style = if self.selection == Selection::CancelButton {
            Style::default().bg(Color::Gray).fg(Color::Black)
        } else {
            Style::default().bg(Color::DarkGray).fg(Color::White)
        };

        let cancel_button = Paragraph::new(" cancel ")
            .block(Block::default().borders(Borders::ALL))
            .style(cancel_style)
            .alignment(Alignment::Center);

        let write_style = if self.selection == Selection::WriteButton {
            Style::default().bg(Color::LightBlue).fg(Color::Black)
        } else {
            Style::default().bg(Color::Indexed(26)).fg(Color::White)
        };

        let write_button = Paragraph::new(" write ")
            .block(Block::default().borders(Borders::ALL))
            .style(write_style)
            .alignment(Alignment::Center);


        frame.render_widget(cancel_button, button_chunks[0]);
        frame.render_widget(write_button, button_chunks[1]);
    }
}

impl WritePopup {
    pub fn new() -> Self {
        Self {
            path: "".to_string(),
            selection: Selection::CancelButton,
        }
    }

    pub fn init(&mut self, fstab: &Fstab) {
        self.selection = Selection::CancelButton;
        self.path = fstab.path.clone();
    }

    fn write_fstab_to_file(&self, fstab: &Fstab) -> Option<PopupAction> {
        match fstab.write_to_file(&self.path) {
            Ok(_) => Some(PopupAction::ShowPopup(crate::app::Popup::WriteSuccessful)),
            Err(_) => Some(PopupAction::ShowPopup(crate::app::Popup::WriteError))
        }
    }
}
