use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::{Constraint, Direction, Layout, Rect, Alignment}, style::{Color, Style}, text::Text, widgets::{Block, Borders, Clear, Padding, Paragraph}};
use crate::{components::TextInput, fstab::{Fstab, FstabLine}, popups::popup::{Popup, PopupAction, get_centered_area}};

#[derive(PartialEq)]
pub enum Selection {
    CommentInput,
    AutoLeadingSpaceCheckbox,
    ConfirmButton,
    CancelButton
}

#[derive(Debug, Clone, Copy)]
pub struct CommentEditPopupData {
    pub fstab_line: Option<usize>
}

pub struct CommentEditPopup {
    comment_input: TextInput,
    selection: Selection,
    auto_leading_space: bool,
    fstab_line: Option<usize>
}

impl Popup for CommentEditPopup {
    fn handle_key_event(&mut self, key_event: KeyEvent, fstab: &mut Fstab) -> Option<PopupAction> {
        match key_event.code {
            KeyCode::Esc => Some(PopupAction::Close),
            KeyCode::Enter => {
                match self.selection {
                    Selection::AutoLeadingSpaceCheckbox => {
                        self.toggle_auto_leading_space();
                        None
                    }
                    Selection::CancelButton => Some(PopupAction::Close),
                    Selection::ConfirmButton => {
                        self.save_data(fstab);
                        Some(PopupAction::Close)
                    },
                    _ => None
                }
            },
            KeyCode::Left => {
                match self.selection {
                    Selection::ConfirmButton => {
                        self.selection = Selection::CancelButton;
                    },
                    Selection::CommentInput => {
                        self.comment_input.left();
                    },
                    _ => {}
                };
                None
            },
            KeyCode::Right => {
                match self.selection {
                    Selection::CancelButton => {
                        self.selection = Selection::ConfirmButton;
                    },
                    Selection::CommentInput => {
                        self.comment_input.right();
                    },
                    _ => {}
                };
                None
            },
            KeyCode::Backspace => {
                match self.selection {
                    Selection::CommentInput => {
                        self.comment_input.backspace();
                    },
                    _ => {}
                };
                None
            },
            KeyCode::Delete => {
                match self.selection {
                    Selection::CommentInput => {
                        self.comment_input.delete();
                    },
                    _ => {}
                };
                None
            },
            KeyCode::Up => {
                match self.selection {
                    Selection::ConfirmButton | Selection::CancelButton => {
                        self.selection = Selection::AutoLeadingSpaceCheckbox;
                    },
                    Selection::AutoLeadingSpaceCheckbox => {
                        self.selection = Selection::CommentInput;
                    }
                    _ => {}
                };
                None
            },
            KeyCode::Down => {
                match self.selection {
                    Selection::CommentInput => {
                        self.selection = Selection::AutoLeadingSpaceCheckbox;
                    },
                    Selection::AutoLeadingSpaceCheckbox => {
                        self.selection = Selection::CancelButton;
                    },
                    _ => {}
                };
                None
            },
            KeyCode::Home => {
                match self.selection {
                    Selection::CommentInput => {
                        self.comment_input.home();
                    },
                    _ => {}
                }
                None
            },
            KeyCode::End => {
                match self.selection {
                    Selection::CommentInput => {
                        self.comment_input.end();
                    },
                    _ => {}
                }
                None
            },
            KeyCode::Char(c) => {
                match self.selection {
                    Selection::CommentInput => {
                        self.comment_input.write(&c);
                        None
                    },
                    _ => match c {
                        'q' => Some(PopupAction::Close),
                        _ => None
                    }
                }
            }
            _ => None
        }
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let popup_area = get_centered_area(80, 13, area);
        frame.render_widget(Clear, popup_area);
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" edit comment ")
            .padding(Padding::new(2, 2, 1, 1));

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(1),
                Constraint::Length(3),
            ]).spacing(1).split(block.inner(popup_area));

        frame.render_widget(block, popup_area);

        let text = Text::from(self.comment_input.get_input_text());

        let comment_input_block = Block::new()
            .borders(Borders::ALL)
            .title(" comment: ");

        if self.selection == Selection::CommentInput {
            let comment_input_block_inner = comment_input_block.inner(chunks[0]);
            let cursor_x = comment_input_block_inner.x + self.comment_input.get_cursor_offset();
            let cursor_y = comment_input_block_inner.y;

            frame.set_cursor_position((cursor_x, cursor_y));
        }

        let comment_input = Paragraph::new(text)
            .block(comment_input_block);

        frame.render_widget(comment_input, chunks[0]);

        let auto_leading_space_checkbox_style = if self.selection == Selection::AutoLeadingSpaceCheckbox {
            Style::default().fg(Color::White)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let auto_leading_space_checkbox_text = if self.auto_leading_space {
            Text::from("[x] - add space before comment").style(auto_leading_space_checkbox_style)
        } else {
            Text::from("[ ] - add space before comment").style(auto_leading_space_checkbox_style)
        };

        let auto_leading_space_checkbox = Paragraph::new(auto_leading_space_checkbox_text);

        frame.render_widget(auto_leading_space_checkbox, chunks[1]);

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

        let confirm_style = if self.selection == Selection::ConfirmButton {
            Style::default().bg(Color::LightGreen).fg(Color::Black)
        } else {
            Style::default().bg(Color::Indexed(22)).fg(Color::White)
        };

        let confirm_button = Paragraph::new(" confirm ")
            .block(Block::default().borders(Borders::ALL))
            .style(confirm_style)
            .alignment(Alignment::Center);


        frame.render_widget(cancel_button, button_chunks[0]);
        frame.render_widget(confirm_button, button_chunks[1]);
    }
}

impl CommentEditPopup {
    pub fn new() -> Self {
        let mut comment_input = TextInput::default();
        comment_input.set_prefix(&"# ".to_string());
        Self {
            comment_input,
            selection: Selection::CommentInput,
            auto_leading_space: true,
            fstab_line: None
        }
    }

    pub fn init(&mut self, data: CommentEditPopupData, fstab: &Fstab) {
        self.selection = Selection::CommentInput;
        self.fstab_line = data.fstab_line;
        if let Some(line) = self.fstab_line {
            match &fstab.lines[line] {
                FstabLine::Comment(comment) => {
                    self.comment_input.set_value(comment);
                },
                _ => {}
            }
        }

        if self.auto_leading_space {
            self.comment_input.set_prefix(&"# ".to_string());
            if let Some(comment) = self.comment_input.value.strip_prefix(" ") {
                self.comment_input.set_value(&comment.to_string());
            }
        } else {
            self.comment_input.set_prefix(&"#".to_string());
        }
    }

    fn save_data(&mut self, fstab: &mut Fstab) {
        if let Some(selected) = self.fstab_line {
            let new_comment = if self.auto_leading_space {
                FstabLine::Comment(" ".to_owned() + self.comment_input.value.as_str())
            } else {
                FstabLine::Comment(self.comment_input.value.clone())
            };

            fstab.lines[selected] = new_comment;
        }
    }

    fn toggle_auto_leading_space(&mut self) {
        self.auto_leading_space = !self.auto_leading_space;
        self.comment_input.set_prefix(&match self.auto_leading_space {
            true => "# ",
            false => "#"
        }.to_string());
    }
}
