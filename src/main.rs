mod bundles;
mod components;
mod resources;
mod tech;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::Rng;
use std::io::Write;
use std::{collections::HashSet, io};
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
        // .add_plugins(MinimalPlugins)
        // for some reason, adding minimal plugins causes the crossterm backend to not render
        .add_startup_system(spawn_galaxy)
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
        terminal.draw(|f| ui(f, &tui_state, &mut app))?;

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

fn ui<B: Backend>(f: &mut Frame<B>, tui_state: &TuiState, app: &mut App) {
    // TODO: 1. Canvas views (Galaxy, AstroObject), Informational popup, galaxy go-to by name (or id)
    let frame_size = f.size();
    let points = get_star_ui_points(app, frame_size);
    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Galaxy"))
        .marker(tui::symbols::Marker::Dot)
        .paint(|ctx| {
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

fn get_star_ui_points(app: &mut App, frame_size: tui::layout::Rect) -> Vec<(f64, f64)> {
    let mut galactic_obj_query = app
        .world
        .query::<(&components::astronomy::GalacticObj, &components::Location)>();
    let config = app
        .world
        .get_resource::<resources::Config>()
        .expect("config not found");
    let mut points = vec![];
    for (_, loc) in galactic_obj_query.iter(&app.world) {
        // Add linear interpolation to scale x, y
        // from range [a,b] (the grid) to range [c,d] (the canvas)
        let x = loc.x as f64;
        let y = loc.y as f64;
        let (a, b) = (0., config.galaxy_dimension as f64 - 1.);
        let (cx, dx) = (0., frame_size.width as f64);
        let (cy, dy) = (0., frame_size.height as f64);
        let fx = |p: f64| cx + ((dx - cx) / (b - a)) * (p - a);
        let fy = |p: f64| cy + ((dy - cy) / (b - a)) * (p - a);
        let point = (fx(x), fy(y));
        log::trace!(
            "scaling astro_grid point ({}, {}) to canvas point ({}, {})",
            x,
            y,
            point.0,
            point.1
        );
        points.push(point);
    }
    points
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

fn spawn_galaxy(mut commands: Commands, config: Res<Config>) {
    let mut rng = rand::thread_rng();
    // TODO: num stars isn't guaranteed to be accurate because any location collisions will be passed
    let mut used_locations = HashSet::new();
    for _ in 0..config.num_stars {
        let x = rng.gen_range(0..config.galaxy_dimension);
        let y = rng.gen_range(0..config.galaxy_dimension);
        let is_used = used_locations.insert((x, y));
        log::trace!("used_locations: {:?}", used_locations);
        if is_used {
            log::trace!("location ({}, {}) is already used", x, y);
            continue;
        }
        let obj = components::astronomy::GalacticObj::Star;
        let ui_offset = (rng.gen_range(-0.1..0.1), rng.gen_range(-0.1..0.1));
        commands.spawn((
            components::Location {
                x,
                y,
                w: 0,
                z: 0,
                ui_offset,
            },
            obj,
        ));
    }
    log::info!("spawned {} stars", config.num_stars);
}

mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let mut app = App::new();
        app.init_resource::<Config>();
        app.update();

        let config = app.world.get_resource::<Config>().unwrap();
        assert_eq!(config.galaxy_dimension, 10);
    }
}
