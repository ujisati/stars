use crate::game;
use rand::{
    distributions::{Distribution, Uniform},
    rngs::ThreadRng,
};
use tui::widgets::ListState;
use tui_tree_widget::{TreeItem, TreeState};

const STARS: &str = "Stars";
const ACTIONS: [&str; 2] = ["View My Stars", "View My Units"];

pub struct StatefulTree<'a> {
    pub state: TreeState,
    pub items: Vec<TreeItem<'a>>,
}

impl<'a> StatefulTree<'a> {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            state: TreeState::default(),
            items: Vec::new(),
        }
    }

    pub fn with_items(items: Vec<TreeItem<'a>>) -> Self {
        Self {
            state: TreeState::default(),
            items,
        }
    }

    pub fn first(&mut self) {
        self.state.select_first();
    }

    pub fn last(&mut self) {
        self.state.select_last(&self.items);
    }

    pub fn down(&mut self) {
        self.state.key_down(&self.items);
    }

    pub fn up(&mut self) {
        self.state.key_up(&self.items);
    }

    pub fn left(&mut self) {
        self.state.key_left();
    }

    pub fn right(&mut self) {
        self.state.key_right();
    }

    pub fn toggle(&mut self) {
        self.state.toggle_selected();
    }
}

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
    pub tree: StatefulTree<'a>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool) -> App<'a> {
        let mut galaxy = game::Galaxy::new();
        let mut game = game::Game::new(vec!["Player 1"]);
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
            tree: StatefulTree::new(),
        }
    }

    pub fn on_up(&mut self) {
        match self.active_tab {
            STARS => {
                self.tree.up();
            }
            _ => {}
        }
    }

    pub fn on_down(&mut self) {
        match self.active_tab {
            STARS => {
                self.tree.down();
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
            _ => {}
        }
    }

    pub fn on_enter(&mut self) {
        match self.active_tab {
            STARS => {
                self.tree.toggle();
            }
            _ => {}
        };
    }

    pub fn on_tick(&mut self) {
        self.tree.items = self
            .game
            .get_players_stars("Player 1")
            .iter()
            .map(|s| {
                TreeItem::new(
                    s.name.clone(),
                    s.planets
                        .iter()
                        .map(|p| TreeItem::new(p.name.clone(), vec![]))
                        .collect::<Vec<TreeItem>>(),
                )
            })
            .collect();
    }
}
