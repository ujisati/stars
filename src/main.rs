mod bundles;
mod components;
mod resources;
mod utilities;

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
use rand::prelude::*;
use resources::*;
use utilities::names::*;

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
        let x = (loc.x as f64 + loc.ui_offset.0 as f64).clamp(0., frame_size.width as f64);
        let y = (loc.y as f64 + loc.ui_offset.1 as f64).clamp(0., frame_size.height as f64);
        let (a, b) = (0., config.galaxy_dimension as f64 - 1.);
        let (cx, dx) = (0., frame_size.width as f64);
        let (cy, dy) = (0., frame_size.height as f64);
        let f_of_x = |p: f64| ((p - a) * ((dx - cx) / (b - a))) + cx;
        let f_of_y = |p: f64| ((p - a) * ((dy - cy) / (b - a))) + cy;
        let canvas_point = (f_of_x(x), f_of_y(y));
        log::trace!(
            "scaling astro_grid point ({}, {}) to canvas point ({}, {})",
            x,
            y,
            canvas_point.0,
            canvas_point.1
        );
        points.push(canvas_point);
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
    // TODO: add randomness and pre-made patterns
    let mut rng = thread_rng();
    let mut star_count = 0;
    let mut used_dimensions = HashSet::new();
    while star_count < config.num_stars {
        let x = rng.gen_range(0..config.galaxy_dimension);
        let y = rng.gen_range(0..config.galaxy_dimension);
        if used_dimensions.contains(&(x, y)) {
            continue;
        }
        used_dimensions.insert((x, y));

        // get ui offset
        let choices = [0.1, 0.2, 0.3];
        let weights = [3, 2, 1];
        let dist = rand::distributions::WeightedIndex::new(&weights).unwrap();

        let ui_offset = (
            choices[dist.sample(&mut rng)],
            choices[dist.sample(&mut rng)],
        );
        let star_name = random_name();
        commands.spawn((
            components::Location {
                x,
                y,
                w: 0,
                z: 0,
                ui_offset,
            },
            components::astronomy::GalacticObj::Star,
            components::Name(star_name.clone()),
        ));
        star_count += 1;
        log::trace!("spawned star {} at ({}, {})", star_name, x, y); 
    }
    log::info!("spawned {} stars", config.num_stars);
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_default_config() {
        let mut app = App::new();
        app.init_resource::<Config>();
        app.update();

        let config = app.world.get_resource::<Config>();
        assert!(config.is_some());
    }

    #[test]
    fn test_spawn_galaxy() {
        let mut app = App::new();
        app.init_resource::<Config>()
            .add_plugins(MinimalPlugins)
            .add_startup_system(spawn_galaxy);

        let mut galactic_obj_query = app.world.query::<&components::astronomy::GalacticObj>();
        assert_eq!(galactic_obj_query.iter(&app.world).count(), 0);

        app.update();
        assert_eq!(galactic_obj_query.iter(&app.world).count(), 100);
    }
}
