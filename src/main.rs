mod bundles;
mod components;
mod resources;
mod tech;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{
        canvas::{Canvas, Painter, Shape},
        Block, Borders, List, ListItem, Paragraph,
    },
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

use bevy::prelude::*;
use log;
use resources::*;

enum InputMode {
    Normal,
    Editing,
}

struct TuiState {
    /// Current value of the input box
    input: String,
    /// Current input mode
    input_mode: InputMode,
    /// History of recorded messages
    messages: Vec<String>,
}

impl Default for TuiState {
    fn default() -> TuiState {
        TuiState {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
        }
    }
}

fn main() {
    env_logger::init();
    log::info!("creating bevy app");
    App::new()
        .set_runner(runner)
        .init_resource::<resources::Config>()
        .init_resource::<resources::Galaxy>()
        // .add_plugins(MinimalPlugins)
        // for some reason, adding minimal plugins causes the crossterm backend to not render
        .run();
}

fn runner(app: App) {
    // setup terminal
    enable_raw_mode().expect("failed to enter raw mode");
    log::info!("crossterm raw mode enabled");
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
        .expect("failed to enter alternate screen or enable mouse capture");
    log::info!("crossterm alternate screen enabled, mouse capture enabled");
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("failed to create terminal backend");
    log::info!("terminal backend created");

    // create game loop and run it
    let tui_state = TuiState::default();
    let res = loop_game(&mut terminal, tui_state, app);

    if let Err(err) = res {
        println!("{:?}", err)
    }

    // restore terminal
    disable_raw_mode().expect("failed to disable raw mode");
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .expect("failed to leave alternate screen or disable mouse capture");
    terminal.show_cursor().expect("failed to show cursor");
}

fn loop_game<B: Backend>(
    terminal: &mut Terminal<B>,
    mut tui_state: TuiState,
    mut app: App,
) -> io::Result<()> {
    loop {
        log::info!("beginning game loop");

        log::info!("drawing ui");
        terminal.draw(|f| ui(f, &tui_state, &app))?;

        log::info!("reading input");
        if let Event::Key(key) = event::read()? {
            match tui_state.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        tui_state.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        tui_state.messages.push(tui_state.input.drain(..).collect());
                    }
                    KeyCode::Char(c) => {
                        tui_state.input.push(c);
                    }
                    KeyCode::Backspace => {
                        tui_state.input.pop();
                    }
                    KeyCode::Esc => {
                        tui_state.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
            }
        }

        // update bevy
        log::info!("updating bevy");
        app.update();
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, tui_state: &TuiState, app: &App) {
    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Galaxy"))
        .paint(|ctx| {
            ctx.draw(app.world.get_resource::<Galaxy>().unwrap());
            // let star_index = app.tree.state.selected()[0];
            // let star = app.game.get_players_stars("Player 1")[star_index];
            // let star_label_point = get_star_label_point(&star);
            // ctx.draw(star);
            // ctx.print(
            //     star_label_point.0,
            //     star_label_point.1,
            //     Span::styled(star.name.clone(), Style::default().fg(Color::White)),
            // );
        })
        .x_bounds([0.0, 100.0])
        .y_bounds([0.0, 100.0]);
    f.render_widget(canvas, f.size());
}

impl Shape for resources::Galaxy {
    fn draw(&self, painter: &mut Painter) {
        for x in 0..self.object_grid.len() {
            for y in 0..self.object_grid[x].len() {
                if let Some(_) = &self.object_grid[x][y] {
                    // TODO: Add linear scale to bound x and y to resolution
                    painter.paint(x, y, Color::Yellow);
                }
            }
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let mut app = App::new();
        app.init_resource::<Config>();
        app.update();

        let config = app.world.get_resource::<Config>().unwrap();
        assert_eq!(config.galaxy_dim, 10);
    }

    #[test]
    fn test_galaxy() {
        let mut app = App::new();
        app.init_resource::<Config>().init_resource::<Galaxy>();
        app.update();

        let galaxy = app.world.get_resource::<Galaxy>().unwrap();
        assert_eq!(galaxy.object_grid.len(), 10);
    }
}
