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

impl Shape for game::Game {
    fn draw(&self, painter: &mut Painter) {
        for star in &self.get_player_visible_stars(self.get_player("Player 1")) {
            let (x, y) = star.location;
            if let Some((x, y)) = painter.get_point(x as f64, y as f64) {
                painter.paint(x, y, Color::Yellow);
            }
        }
    }
}

impl Shape for game::Star {
    fn draw(&self, painter: &mut Painter) {
        let (x, y) = self.location;
        if let Some((x, y)) = painter.get_point(x as f64, y as f64) {
            painter.paint(x, y, Color::Red);
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
        .block(Block::default().borders(Borders::ALL).title("Galaxy"))
        .paint(|ctx| {
            ctx.draw(&app.game);
            let star_index = app.tree.state.selected()[0];
            let star = app.game.get_players_stars("Player 1")[star_index];
            let star_label_point = get_star_label_point(&star);
            ctx.draw(star);
            ctx.print(
                star_label_point.0,
                star_label_point.1,
                Span::styled(star.name.clone(), Style::default().fg(Color::White)),
            );
        })
        .x_bounds([0.0, 100.0])
        .y_bounds([0.0, 100.0]);
    f.render_widget(canvas, area);
}

fn get_star_label_point(star: &game::Star) -> (f64, f64) {
    let (mut x, mut y) = star.location;
    x = x.clamp(1, 99) + 1;
    y = y.clamp(1, 99) + 1;
    (x as f64, y as f64)
}
