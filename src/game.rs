extern crate nalgebra as na;
use crate::utils;
use log::{info, warn};
use memmap::Mmap;
use rand::prelude::*;
use std::fmt;
use std::fs;
use std::io::{Error, Write};
use std::{collections::HashMap, fmt::Display};

pub struct Game {
    pub galaxy: Galaxy,
    pub players: Vec<Player>,
    pub turn: u32,
    pub game_over: bool,
}

impl Game {
    pub fn new(players: Vec<&str>) -> Self {
        let mut galaxy = Galaxy::new();
        galaxy.populate_default();
        let players = players
            .iter()
            .map(|player| Player {
                name: player.to_string(),
            })
            .collect();

        let mut game = Game {
            galaxy,
            players,
            turn: 0,
            game_over: false,
        };
        game
    }

    pub fn set_players_start(&mut self) {
        for player in &self.players {
            for _ in 0..5 {
                let location = self.random_unoccupied_habitable_planet();
                let planet = self.galaxy.get_planet_from_loc(location);
                let unit = Unit {
                    player: player.name.clone(),
                    size: 1,
                };
                planet.units.push(unit);
            }
        }
    }

    pub fn get_players_units(&self, player: &Player) -> Vec<&Unit> {
        let mut units = Vec::new();
        for star in &self.galaxy.stars {
            for planet in &star.planets {
                for unit in &planet.units {
                    if unit.player == player.name {
                        units.push(unit);
                    }
                }
            }
        }
        units
    }

    pub fn get_players_stars(&self, player: &str) -> Vec<&Star> {
        let mut stars = Vec::new();
        for star in &self.galaxy.stars {
            for planet in &star.planets {
                for unit in &planet.units {
                    if unit.player == player {
                        stars.push(star);
                        break;
                    }
                }
            }
        }
        stars
    }

    pub fn random_unoccupied_habitable_planet(&self) -> (usize, usize, usize) {
        loop {
            let star = &self.galaxy.stars[random::<usize>() % self.galaxy.total_stars];
            if star.planets.len() == 0 {
                continue;
            }
            let planet = &star.planets[random::<usize>() % star.planets.len()];
            if planet.units.len() == 0 && planet.habitable {
                return (star.location.0, star.location.1, planet.location.2);
            }
        }
    }
}

#[derive(Debug)]
pub struct Galaxy {
    pub dimensions: (usize, usize),
    pub stars: Vec<Star>,
    pub star_matrix: na::SMatrix<usize, 100, 100>,
    pub distances: HashMap<usize, Vec<(usize, u32)>>,
    pub total_stars: usize,
}

impl Galaxy {
    pub fn new() -> Self {
        const X_DIM: usize = 100;
        const Y_DIM: usize = 100;
        let scale_factor = 10;
        Galaxy {
            dimensions: (X_DIM, X_DIM),
            stars: Vec::new(),
            star_matrix: na::SMatrix::<usize, X_DIM, X_DIM>::zeros(),
            distances: HashMap::new(),
            total_stars: X_DIM * X_DIM / scale_factor,
        }
    }

    pub fn get_star_from_loc(&mut self, location: (usize, usize)) -> &mut Star {
        let index = self.star_matrix[(location.0, location.1)];
        &mut self.stars[index]
    }

    pub fn get_planet_from_loc(&mut self, location: (usize, usize, usize)) -> &mut Planet {
        let star = self.get_star_from_loc((location.0, location.1));
        &mut star.planets[location.2]
    }

    fn add_star(&mut self, star: Star, index: usize) -> Result<(), &str> {
        // Check if the star is already in the matrix
        // If it is, throw an error
        if self.star_matrix[(star.location.0, star.location.1)] != 0 {
            info!(
                "Star already exists at location ({}, {})",
                star.location.0, star.location.1
            );
            return Err("Star already exists at location");
        }
        self.star_matrix[(star.location.0, star.location.1)] = index;
        info!(
            "Star {} added at location ({}, {})",
            star.name, star.location.0, star.location.1
        );
        self.stars.push(star);
        Ok(())
    }

    fn get_star_index(&self, star: &Star) -> usize {
        self.star_matrix[(star.location.0, star.location.1)]
    }

    pub fn populate_default(&mut self) {
        for i in 0..self.total_stars {
            let mut valid_location = false;
            while !valid_location {
                let x = random::<usize>() % self.dimensions.0;
                let y = random::<usize>() % self.dimensions.1;
                let num_planets = random::<usize>() % 10;
                let name = utils::random_name();
                let mut star = Star {
                    name: name.clone(),
                    planets: Vec::new(),
                    location: (x, y),
                };
                for z in 0..num_planets {
                    let mut planet_name = name.clone();
                    planet_name.push_str("-");
                    planet_name.push_str(&z.to_string());
                    let planet = Planet {
                        name: planet_name,
                        location: (x, y, z),
                        habitable: random::<bool>(),
                        units: Vec::new(),
                    };
                    info!("Planet {} added to star {}", planet.name, star.name);
                    star.planets.push(planet);
                }
                valid_location = match self.add_star(star, i) {
                    Ok(_) => true,
                    Err(_) => false,
                };
            }
        }
        self.calculcate_all_distances();
        info!("Galaxy populated with {} stars", self.total_stars);
    }

    fn calculcate_all_distances(&mut self) {
        //iterate over all stars references
        for (i, star) in self.stars.iter().enumerate() {
            let mut distances_to = Vec::new();
            for other_star in self.stars.iter() {
                if star.name == other_star.name {
                    continue;
                }
                let distance = Star::distance_between(star, other_star);
                let star_and_distance = (self.get_star_index(other_star), distance);
                distances_to.push(star_and_distance);
            }
            distances_to.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            self.distances.insert(i, distances_to);
        }
    }

    fn get_stars_within_range(&self, star: &Star, range: u32) -> Vec<&Star> {
        let mut stars = Vec::new();
        let star_index = self.get_star_index(star);
        let distances: &Vec<(usize, u32)> = &self.distances[&star_index];
        for distance in distances {
            if distance.1 <= range {
                stars.push(&self.stars[distance.0]);
            }
        }
        stars
    }

    pub fn get_star_by_name(&self, name: &str) -> Option<&Star> {
        for star in self.stars.iter() {
            if star.name == name {
                return Some(star);
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct Star {
    pub name: String,
    pub planets: Vec<Planet>,
    pub location: (usize, usize),
}

impl fmt::Display for Star {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {:?}", self.name, self.location)
    }
}

impl Star {
    fn distance_between(star1: &Star, star2: &Star) -> u32 {
        let x1 = star1.location.0 as f32;
        let y1 = star1.location.1 as f32;
        let x2 = star2.location.0 as f32;
        let y2 = star2.location.1 as f32;
        let distance = ((x1 - y1).powi(2) + (x2 - y2).powi(2)).sqrt() as u32;
        info!(
            "Distance between {} and {} is {} SpU",
            star1.name, star2.name, distance
        );
        distance
    }
}

#[derive(Debug)]
pub struct Planet {
    pub name: String,
    pub location: (usize, usize, usize),
    pub habitable: bool,
    pub units: Vec<Unit>,
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
}

#[derive(Debug)]
pub struct Unit {
    pub size: u32,
    pub player: String,
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[fixture]
//     fn game() -> Game {
//         let mut players = vec!["Player 1", "Player 2"];
//         let mut game = Game::new(players);
//         game.set_players_start();
//         game
//     }
//
//     #[test]
//     fn test() {
//         // Find the distance between two total_stars
//         let star1 = &game.galaxy.stars[0];
//         let star2 = &game.galaxy.stars[1];
//         let distance = Star::distance_between(star1, star2);
//         info!("Distances {:?}", game.galaxy.distances[&0]);
//
//         // Get all stars within range
//         let stars = game.galaxy.get_stars_within_range(star1, 100);
//         assert!(stars.len() > 0);
//
//         // Get players stars
//         let player_stars = game.get_players_stars("Player 1");
//         assert!(player_stars.len() > 0);
//     }
// }
