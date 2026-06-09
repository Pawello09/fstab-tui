use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::{Constraint, Direction, Layout, Rect, Alignment}, style::{Color, Style}, widgets::{Block, Borders, Clear, Padding, Paragraph}};
use crate::{fstab::Fstab, popups::popup::{Popup, PopupAction, get_centered_area}};

pub struct WriteSuccessfulPopup {
    path: String
}

impl Popup for WriteSuccessfulPopup {
    fn handle_key_event(&mut self, key_event: KeyEvent, _fstab: &mut Fstab) -> Option<PopupAction> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Enter => Some(PopupAction::Close),
            _ => None
        }
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let popup_area = get_centered_area(60, 9, area);
        frame.render_widget(Clear, popup_area);
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" write successful ")
            .padding(Padding::new(2, 2, 1, 1));

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(3),
            ]).spacing(1).split(block.inner(popup_area));

        frame.render_widget(block, popup_area);

        let title = Paragraph::new("fstab successfully written to path: ".to_owned() + &self.path).style(Style::new().bold()).centered();

        frame.render_widget(title, chunks[0]);

        let ok_button = Paragraph::new(" ok ")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().bg(Color::LightBlue).fg(Color::Black))
            .alignment(Alignment::Center);


        frame.render_widget(ok_button, chunks[1]);
    }
}

impl WriteSuccessfulPopup {
    pub fn new() -> Self {
        Self {
            path: "".to_string(),
        }
    }

    pub fn init(&mut self, fstab: &Fstab) {
        self.path = fstab.path.clone();
    }
}
