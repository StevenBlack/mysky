mod app;
mod astronomy;
mod ui;

use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use app::App;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let tick = Duration::from_secs(30);
    let mut last_tick = Instant::now();
    let mut app = App::new();

    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        let timeout = tick.checked_sub(last_tick.elapsed()).unwrap_or_default();
        if event::poll(timeout)?
            && let Event::Key(key) = event::read()?
                && key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Char('u') => app.toggle_utc(),
                        KeyCode::Char('r') => app.update(),
                        _ => {}
                    }
                }

        if last_tick.elapsed() >= tick {
            app.update();
            last_tick = Instant::now();
        }
    }
}
