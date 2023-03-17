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
use std::io::Write;
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
    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            let mut level_style = buf.style();
            let style = match record.level() {
                log::Level::Error => level_style
                    .set_color(env_logger::fmt::Color::Red)
                    .set_bold(true),
                log::Level::Warn => level_style
                    .set_color(env_logger::fmt::Color::Yellow)
                    .set_bold(true),
                log::Level::Info => level_style
                    .set_color(env_logger::fmt::Color::Green)
                    .set_bold(true),
                log::Level::Debug => level_style
                    .set_color(env_logger::fmt::Color::Magenta)
                    .set_bold(true),
                log::Level::Trace => level_style
                    .set_color(env_logger::fmt::Color::Blue)
                    .set_bold(true),
            };
            writeln!(
                buf,
                "{}:{} {} [{}] {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                style.value(record.level()),
                record.args()
            )
        })
        .init();
    log::info!("~~~ welcome to STARS ~~~");
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
    // TODO: 1. Canvas views (Galaxy, AstroObject), Informational popup, galaxy go-to by name (or id)
    let frame_size = f.size();
    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Galaxy"))
        .marker(tui::symbols::Marker::Dot)
        .paint(|ctx| {
            let galaxy = app.world.get_resource::<Galaxy>().unwrap();
            let mut points = vec![];
            for x in 0..galaxy.object_grid.len() {
                for y in 0..galaxy.object_grid[x].len() {
                    if let Some(_) = &galaxy.object_grid[x][y] {
                        // TODO: Add linear interpolation to scale x, y
                        // from range [a,b] (the grid) to range [c,d] (the canvas)
                        let (a, b) = (0., (galaxy.object_grid.len() - 1) as f64);
                        let (cx, dx) = (0., frame_size.width as f64);
                        let (cy, dy) = (0., frame_size.height as f64);
                        let fx = |p: f64| cx + ((dx - cx) / (b - a)) * (p - a);
                        let fy = |p: f64| cy + ((dy - cy) / (b - a)) * (p - a);
                        let point = (fx(x as f64), fy(y as f64));
                        log::trace!(
                            "scaling astro_grid point ({}, {}) to canvas point ({}, {})",
                            x,
                            y,
                            point.0,
                            point.1
                        );
                        points.push(point);
                    }
                }
            }
            ctx.draw(&Points {
                coords: &points,
                color: Color::Yellow,
            });
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
        .x_bounds([0., f.size().width as f64])
        .y_bounds([0., f.size().height as f64]);

    f.render_widget(canvas, f.size());
}

/// A shape to draw a group of points with the given color
#[derive(Debug, Clone)]
pub struct Points<'a> {
    pub coords: &'a Vec<(f64, f64)>,
    pub color: Color,
}

impl<'a> Shape for Points<'a> {
    fn draw(&self, painter: &mut Painter) {
        for (x, y) in self.coords {
            if let Some((x, y)) = painter.get_point(*x, *y) {
                painter.paint(x, y, self.color);
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
