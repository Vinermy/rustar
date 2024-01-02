mod f_event;
mod menu_item;
mod ui;
mod style;

use f_event::Event as FEvent;
use menu_item::MenuItem;
use ui::menu_bar::render_menu_bar;

use std::{io, thread};
use std::sync::mpsc;
use std::time::Instant;
use chrono::Duration;
use crossterm::event::{EnableMouseCapture, KeyCode};
use crossterm::{event, execute};
use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use tui::backend::CrosstermBackend;
use tui::Terminal;
use crossterm::event::Event as CEvent;
use tui::layout::{Constraint, Direction, Layout};
use crate::f_event::Event;

fn main() {
    enable_raw_mode().expect("");
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).expect("");
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)
        .expect("");
    terminal.clear().expect("Can clear terminal");

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::milliseconds(200);

    thread::spawn(move || { // Input-capturing thread
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(&Duration::from_std(last_tick.elapsed())
                    .expect("Can convert from std::Duration"))
                .unwrap_or_else(|| Duration::seconds(0));

            if event::poll(timeout.to_std().expect(""))
                .expect("poll works") {
                if let CEvent::Key(key) = event::read()
                    .expect("can read events") {
                    tx.send(
                        match key.code {
                            KeyCode::Backspace => { FEvent::Backspace }
                            KeyCode::Enter => { FEvent::Enter }
                            KeyCode::Left => { FEvent::MoveLeft }
                            KeyCode::Right => { FEvent::MoveRight }
                            KeyCode::Up => { FEvent::MoveUp }
                            KeyCode::Down => { FEvent::MoveDown }
                            KeyCode::Tab => { FEvent::Tab }
                            KeyCode::F(char) => { FEvent::F(char) }
                            KeyCode::Char(char) => { FEvent::Char(char) }
                            KeyCode::Esc => { FEvent::Escape }
                            _ => { FEvent::Tick }
                        }
                    ).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate.to_std().expect("") {
                if let Ok(_) = tx.send(FEvent::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let menu_titles = vec![
        "Main", "Ships", "Planets"
    ]; // Stores all menu tabs
    let mut active_menu_item = MenuItem::Main;

    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(2),
                    ]
                        .as_ref(),
                )
                .split(size);

            rect.render_widget(
                render_menu_bar(&menu_titles, &active_menu_item),
                chunks[0]);
        }).expect("Can draw");

        match rx.recv().expect("Input received") {
            FEvent::Tick => {}

            Event::MoveUp => {}
            Event::MoveDown => {}
            Event::MoveLeft => {}
            Event::MoveRight => {}
            Event::Backspace => {}
            Event::Tab => {
                active_menu_item = active_menu_item.next();
            }
            Event::Enter => {}
            Event::Escape => {}
            Event::Char(_) => {}
            Event::F(_) => {}
        }

    }
}
