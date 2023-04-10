use tui::{
    backend::Backend,
    layout::{self, Constraint, Layout},
    style::Color,
    text::Spans,
    widgets::{
        canvas::{Canvas, Painter, Shape, Line},
        Block, Borders, Clear, Paragraph,
    },
    Frame,
};

use bevy::prelude::App;
use log;

use crate::{components as cmp, resources};

pub struct TuiState {
    pub active_modal: Modal,
    pub active_view: View,
    pub galaxy_view: GalaxyView,
}

#[derive(PartialEq)]
pub enum View {
    Galaxy,
}

#[derive(PartialEq)]
pub enum Modal {
    Help,
    SearchObj,
    Off,
}

pub struct GalaxyView {
    pub astro_objs: Vec<(u32, u32)>,
    pub selected_astro_obj: Option<(u32, u32)>,
    pub selected_idx: usize,
    pub target_astro_obj: Option<(u32, u32)>,
    pub show_ids: bool,
    pub origin: (f64, f64),
    pub scale: f64,
}

impl Default for GalaxyView {
    fn default() -> GalaxyView {
        GalaxyView {
            astro_objs: Vec::new(),
            selected_astro_obj: None,
            selected_idx: 0,
            target_astro_obj: None,
            show_ids: false,
            origin: (0., 0.),
            scale: 1.,
        }
    }
}

impl TuiState {
    pub fn new(app: &mut App) -> TuiState {
        let astro_objs: Vec<(u32, u32)> = app
            .world
            .query::<(&cmp::astronomy::GalacticObj, &cmp::Location)>()
            .iter(&app.world)
            .map(|(_, loc)| (loc.x, loc.y))
            .collect();
        TuiState {
            galaxy_view: GalaxyView {
                selected_idx: 0,
                selected_astro_obj: Some(astro_objs[0]),
                astro_objs,
                ..GalaxyView::default()
            },
            ..TuiState::default()
        }
    }
}

impl Default for TuiState {
    fn default() -> TuiState {
        TuiState {
            active_modal: Modal::Off,
            active_view: View::Galaxy,
            galaxy_view: GalaxyView {
                ..GalaxyView::default()
            },
        }
    }
}

pub fn ui<B: Backend>(f: &mut Frame<B>, tui_state: &TuiState, app: &mut App) {
    // TODO: 1. Canvas views (Galaxy, AstroObject), Informational popup, galaxy go-to by name (or id)
    let frame_size = f.size();
    let points = get_star_ui_points(
        app,
        frame_size,
        tui_state.galaxy_view.origin,
        tui_state.galaxy_view.scale,
    );
    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Galaxy"))
        .marker(tui::symbols::Marker::Braille)
        .paint(|ctx| {
            ctx.draw(&Points {
                coords: &points,
                color: Color::Yellow,
                selected_astro_obj: tui_state.galaxy_view.selected_astro_obj,
            });
            ctx.draw(&Line {
                x1: 0.,
                y1: 10.,
                y2: 10.,
                x2: 0.,
                color: Color::LightBlue,
            });
        })
        .x_bounds([0., f.size().width as f64])
        .y_bounds([0., f.size().height as f64]);

    f.render_widget(canvas, f.size());

    if tui_state.active_modal == Modal::Help {
        let block = Block::default().title("Help").borders(Borders::ALL);
        let area = centered_rect(60, 20, f.size());
        // add text to the area
        let text = vec![
            Spans::from("Press 'q' to quit"),
            Spans::from("Press 'h' to toggle this help menu"),
        ];
        let paragraph = Paragraph::new(text.clone()).block(block);
        f.render_widget(Clear, area); //this clears out the background
        f.render_widget(paragraph, area);
    }

    //
}

pub fn get_star_ui_points(
    app: &mut App,
    frame_size: tui::layout::Rect,
    origin: (f64, f64),
    scale: f64,
) -> Vec<(&cmp::Location, (f64, f64))> {
    let mut galactic_obj_query = app
        .world
        .query::<(&cmp::astronomy::GalacticObj, &cmp::Location)>();
    let config = app
        .world
        .get_resource::<resources::Config>()
        .expect("config not found");
    let mut points = vec![];
    // TODO: to create a scale, we basically increase the bounds (they will extend past frame size but still from 0,0)
    // TODO: to create a center, we need to offset the origin by the x and y distance of the obj from the center
    // TODO: to move camera, we offset the origin
    // TODO: SOME STARS ARE BEHIND OTHERS BECAUSE IT SEEMS IN SOME POSITIONS THEY COLLAPSE INTO ONE POINT
    // SCALE IS SHEARING OR SOMETHING
    for (_, loc) in galactic_obj_query.iter(&app.world) {
        // Add linear interpolation to scale x, y (world coords + ui offset)
        // from range [a,b] (the grid) to range [c,d] (the canvas)
        let x = (loc.x as f64 + loc.ui_offset.0 as f64);
        let y = (loc.y as f64 + loc.ui_offset.1 as f64);
        let (a, b) = (-10., config.galaxy_dimension as f64 + 10.);
        let (cx, dx) = (origin.0, (frame_size.width as f64 + origin.0) * scale);
        let (cy, dy) = (origin.1, (frame_size.height as f64 + origin.1) * scale);
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
        points.push((loc, canvas_point));
    }
    points
}

#[derive(Debug, Clone)]
pub struct Points<'a> {
    pub coords: &'a Vec<(&'a cmp::Location, (f64, f64))>,
    pub color: Color,
    pub selected_astro_obj: Option<(u32, u32)>,
}

impl<'a> Shape for Points<'a> {
    fn draw(&self, painter: &mut Painter) {
        let astro_obj_selected = self.selected_astro_obj.is_some();
        for (loc, point) in self.coords {
            let mut color = self.color;
            if astro_obj_selected && self.selected_astro_obj.unwrap() == (loc.x, loc.y) {
                log::trace!("selected astro obj at ({}, {})", loc.x, loc.y);
                color = Color::Red;
            }
            if let Some((x, y)) = painter.get_point(point.0, point.1) {
                painter.paint(x, y, color);
            }
        }
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: layout::Rect) -> layout::Rect {
    let popup_layout = Layout::default()
        .direction(layout::Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(layout::Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
