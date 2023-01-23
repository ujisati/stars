use crate::game;
use rand::{
    distributions::{Distribution, Uniform},
    rngs::ThreadRng,
};
use tui::widgets::ListState;

const STARS: &str = "Stars";
const ACTIONS: [&str; 2] = ["View My Stars", "View My Units"];

pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if self.items.len() == 0 {
                    0
                } else if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if self.items.len() == 0 {
                    0
                } else if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub struct App<'a> {
    pub title: &'a str,
    pub game: game::Game,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub actions: StatefulList<&'a str>,
    pub my_stars: StatefulList<String>,
    pub enhanced_graphics: bool,
    pub active_tab: &'a str,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool) -> App<'a> {
        let mut galaxy = game::Galaxy::new();
        let mut game = game::Game::new(vec!["Player 1".to_string()]);
        game.set_players_start();
        App {
            title,
            game,
            should_quit: false,
            tabs: TabsState::new(vec![STARS]),
            actions: StatefulList::with_items(ACTIONS.to_vec()),
            my_stars: StatefulList {
                state: ListState::default(),
                items: Vec::new(),
            },
            enhanced_graphics,
            active_tab: STARS,
        }
    }

    pub fn on_up(&mut self) {
        match self.active_tab {
            STARS => {
                self.my_stars.previous();
            }
            _ => {}
        }
    }

    pub fn on_down(&mut self) {
        match self.active_tab {
            STARS => {
                self.my_stars.next();
            }
            _ => {}
        }
    }

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            't' => {}
            ' ' => match self.active_tab {
                STARS => {}
                _ => {}
            },
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {
        // set my_stars
        let mut my_stars = Vec::new();
        for star in self.game.get_players_stars(&self.game.players[0]) {
            my_stars.push(star.name.to_string());
        }
        self.my_stars.items = my_stars;
    }
}
