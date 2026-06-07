mod app;
mod fstab;
mod screens;
mod ui;
mod handler;
mod popups;

use std::io::Stdout;
use std::{io, env};
use app::App;
use crossterm::event::DisableMouseCapture;
use crossterm::terminal::{LeaveAlternateScreen, disable_raw_mode};
use crossterm::execute;
use ratatui::Terminal;
use ratatui::crossterm::event::EnableMouseCapture;
use ratatui::crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use ratatui::backend::{CrosstermBackend, Backend};

use crate::handler::handle_key_event;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: fstab-tui <file>");
        println!();
        println!("Display and manipulate fstab files with TUI.");
        return Ok(());
    }

    let mut terminal = setup_terminal()?;
    let mut app = App::new(args[1].as_str());

    let res = run_app(&mut terminal, &mut app);

    restore_terminal(&mut terminal)?;
    res
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<(), std::io::Error> where std::io::Error: From<B::Error> {
    while !app.exited {
        terminal.draw(|frame| ui::render(app, frame))?;
        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                handle_key_event(key, app);
            }
        }
    }
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, std::io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture
    )?;

    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), std::io::Error> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
