use crate::{app::App, game};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle},
    widgets::canvas::{Painter, Shape},
    widgets::{
        Axis, BarChart, Block, Borders, Cell, Chart, Dataset, Gauge, LineGauge, List, ListItem,
        Paragraph, Row, Sparkline, Table, Tabs, Wrap,
    },
    Frame,
};
use tui_tree_widget::{Tree, TreeItem};

impl Shape for game::Galaxy {
    fn draw(&self, painter: &mut Painter) {
        for star in &self.stars {
            let (x, y) = star.location;
            if let Some((x, y)) = painter.get_point(x as f64, y as f64) {
                painter.paint(x, y, Color::Yellow);
            }
        }
    }
}

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(app.title))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.index);
    f.render_widget(tabs, chunks[0]);
    match app.tabs.index {
        0 => draw_first_tab(f, app, chunks[1]),
        _ => draw_second_tab(f, app, chunks[1]),
    };
}

fn draw_first_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    // let chunks = Layout::default()
    //     .direction(Direction::Horizontal)
    //     .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
    //     .split(area);
    let block = Block::default().borders(Borders::ALL).title("My Stars");
    f.render_widget(block, area);
    let items = Tree::new(app.tree.items.clone())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Tree Widget {:?}", app.tree.state)),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");
    f.render_stateful_widget(items, area, &mut app.tree.state);
}

fn draw_second_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("World"))
        .paint(|ctx| {
            ctx.draw(&app.game.galaxy);
        })
        .x_bounds([0.0, area.width as f64])
        .y_bounds([0.0, area.height as f64]);
    f.render_widget(canvas, area);
}

// TODO
// CANVAS POINT CLOUD OF THE GALAXY
// TREE VIEW OF OWNED STARS
// Sole control vs contested
