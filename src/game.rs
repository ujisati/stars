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
    pub units: Vec<Unit>,
}

impl Game {
    pub fn new(players: Vec<String>) -> Self {
        let mut galaxy = Galaxy::new();
        galaxy.populate_default();
        let players = players
            .iter()
            .map(|player| Player {
                name: player.clone(),
            })
            .collect();

        let mut game = Game {
            galaxy,
            players,
            turn: 0,
            game_over: false,
            units: Vec::new(),
        };
        game
    }

    pub fn set_players_start(&mut self) {
        let mut units = Vec::new();
        for player in &self.players {
            let planet = self.galaxy.random_unoccupied_planet();
            let unit = Unit {
                player: player.name.clone(),
                location: planet.location,
                size: 1,
            };
            units.push(unit);
        }
        self.units = units;
    }

    pub fn get_players_units(&self, player: &Player) -> Vec<&Unit> {
        let mut units = Vec::new();
        for unit in &self.units {
            if unit.player == player.name {
                units.push(unit)
            }
        }
        units
    }

    pub fn get_players_stars(&self, player: &Player) -> Vec<&Star> {
        let mut stars = Vec::new();
        for unit in self.get_players_units(player) {
            stars.push(self.galaxy.get_star_from_loc(unit.location))
        }
        stars
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
        Galaxy {
            dimensions: (100 as usize, 100 as usize),
            stars: Vec::new(),
            star_matrix: na::SMatrix::<usize, 100, 100>::zeros(),
            distances: HashMap::new(),
            total_stars: 100 * 100 / 100,
        }
    }

    pub fn random_unoccupied_planet(&self) -> &Planet {
        loop {
            let index = random::<usize>() % self.total_stars;
            let star = &self.stars[index];
            if star.planets.len() > 0 {
                let planet_index = random::<usize>() % star.planets.len();
                return &star.planets[planet_index];
            }
        }
    }

    pub fn get_star_from_loc(&self, location: (usize, usize)) -> &Star {
        let index = self.star_matrix[(location.0, location.1)];
        &self.stars[index]
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
                for i in 0..num_planets {
                    let mut planet_name = name.clone();
                    planet_name.push_str("-");
                    planet_name.push_str(&i.to_string());
                    let planet = Planet {
                        name: planet_name.clone(),
                        location: (x, y),
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
                if star == other_star {
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
}

#[derive(Debug)]
pub struct Star {
    pub name: String,
    pub planets: Vec<Planet>,
    pub location: (usize, usize),
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

impl PartialEq for Star {
    fn eq(&self, other: &Star) -> bool {
        self.name == other.name
    }
}

#[derive(Debug)]
pub struct Planet {
    pub name: String,
    pub location: (usize, usize),
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
}

#[derive(Debug)]
pub struct Unit {
    pub location: (usize, usize),
    pub size: u32,
    pub player: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // Create a new galaxy
        let mut galaxy = Galaxy::new();
        galaxy.populate_default();

        // Create a new Player
        let player = Player {
            name: "Player 1".to_string(),
        };

        // Find the distance between two total_stars
        let star1 = &galaxy.stars[0];
        let star2 = &galaxy.stars[1];
        let distance = Star::distance_between(star1, star2);
        info!("Distances {:?}", galaxy.distances[&0]);
    }
}
