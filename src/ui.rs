use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle},
    widgets::{
        Axis, BarChart, Block, Borders, Cell, Chart, Dataset, Gauge, LineGauge, List, ListItem,
        Paragraph, Row, Sparkline, Table, Tabs, Wrap,
    },
    Frame,
};

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
        _ => {}
    };
}

fn draw_first_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
        .split(area);
    let block = Block::default().borders(Borders::ALL).title("My Stars");
    f.render_widget(block, chunks[0]);
    let block = Block::default().borders(Borders::ALL).title("Star Info");
    f.render_widget(block, chunks[1]);

    // Draw actions
    // let actions: Vec<ListItem> = app
    //     .actions
    //     .items
    //     .iter()
    //     .map(|i| ListItem::new(vec![Spans::from(Span::raw(*i))]))
    //     .collect();
    // let actions = List::new(actions)
    //     .block(Block::default().borders(Borders::ALL).title("List"))
    //     .highlight_style(Style::default().add_modifier(Modifier::BOLD))
    //     .highlight_symbol("> ");
    // f.render_stateful_widget(actions, chunks[0], &mut app.actions.state);

    // if "View My Stars" is selected, draw the list stars where player controls atleast 1 planet
    let stars: Vec<ListItem> = app
        .game
        .get_players_stars(&app.game.players[0])
        .iter()
        .map(|i| ListItem::new(i.name.clone()))
        .collect();
    let stars = List::new(stars)
        .block(Block::default().borders(Borders::ALL).title("Stars"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");
    f.render_stateful_widget(stars, chunks[0], &mut app.my_stars.state);

    // Render star info of selected star
    if app.my_stars.items.iter().count() < 1 {
        return;
    }
    let selected_star = app
        .game
        .galaxy
        .get_star_by_name(&app.my_stars.items[app.my_stars.state.selected().unwrap_or(0)].clone())
        .unwrap();
    // create a list of planets in the selected star
    let star_info: Vec<ListItem> = selected_star
        .planets
        .iter()
        .map(|i| ListItem::new(i.name.clone()))
        .collect();
    let star_info = List::new(star_info)
        .block(Block::default().borders(Borders::ALL).title("Star Info"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");
    f.render_widget(star_info, chunks[1]);
}
