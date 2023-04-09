mod bundles;
mod components;
mod keymaps;
mod resources;
mod ui;
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

use components as cmp;

enum InputMode {
    Normal,
    Editing,
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
        .init_resource::<resources::NameGenerator>()
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

    let res = loop_game(&mut terminal, app);
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

fn loop_game<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    log::info!("first bevy update");
    app.update();

    log::info!("initializing ui");
    let mut tui_state = ui::TuiState::new(&mut app);

    log::info!("beginning game loop");
    loop {
        log::info!("drawing ui");

        terminal.draw(|f| ui::ui(f, &tui_state, &mut app))?;
        log::info!("reading input");
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                log::info!("quitting game");
                // TODO: autosave
                return Ok(());
            }
            keymaps::handle_key_event(key, &mut tui_state, &mut app);
        }

        log::info!("updating bevy");
        app.update();
    }
}

fn spawn_galaxy(mut commands: Commands, config: Res<Config>, mut name_generator: ResMut<NameGenerator>) {
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
        let choices = [0.5, 1., 2.];
        let weights = [1, 2, 3];
        let dist = rand::distributions::WeightedIndex::new(&weights).unwrap();
        let is_negative = if rng.gen_bool(0.5) { -1.0 } else { 1.0 };

        let ui_offset = (
            choices[dist.sample(&mut rng)] * is_negative,
            choices[dist.sample(&mut rng)] * is_negative,
        );
        let star_name = name_generator.random_name();
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
