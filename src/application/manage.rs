use crate::presentation::ui;
use crate::{
    application::app::App, domain::system::scan::scan_vscode_workspacestorage_from_system,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

pub async fn run(show_splash_screen: bool) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create ruscode app and run it
    let app = App::new(" Ruscode ", show_splash_screen);
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        match app.status {
            // Exit the app without any error
            super::app::ApplicationStatus::Quit => {
                return Ok(());
            }
            super::app::ApplicationStatus::Running => {
                terminal.draw(|f| ui::draw(f, &mut app))?;

                if crossterm::event::poll(Duration::from_secs(0))? {
                    if let Event::Key(key) = event::read()? {
                        match key.code {
                            KeyCode::Esc => app.on_escape_application(),
                            KeyCode::Tab => app.next_tab(),
                            KeyCode::Up => app.on_up(),
                            KeyCode::Down => app.on_down(),
                            KeyCode::Char(c) => app.on_key(c),
                            KeyCode::Enter => app.enter_in_workspace(),
                            _ => {}
                        }
                    }
                }

                // if last_tick.elapsed() >= tick_rate {
                //     app.on_tick();
                //     last_tick = Instant::now();
                // }
            }
            super::app::ApplicationStatus::SplashScreenReveal => {
                terminal.draw(|f| ui::draw(f, &mut app))?;
                if last_tick.elapsed() >= Duration::from_secs(1) {
                    app.state_change(super::app::ApplicationStatus::Running)
                }
            }
            super::app::ApplicationStatus::SyncVSCode => {
                let current_workspaces = scan_vscode_workspacestorage_from_system();

                // println!("{:?}", current_workspaces);
                if app.show_splash_screen {
                    app.state_change(super::app::ApplicationStatus::SplashScreenReveal)
                } else {
                    app.state_change(super::app::ApplicationStatus::Running)
                }
            }
        }
    }
}
