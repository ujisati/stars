extern crate nalgebra as na;
use crate::utils;
use log::{info, warn};
use memmap::Mmap;
use rand::prelude::*;
use std::fmt;
use std::fs;
use std::io::{Error, Write};
use std::{collections::HashMap, fmt::Display};

pub struct Game<'a> {
    pub galaxy: Galaxy<'a>,
    pub turn: u32,
    pub players: Vec<Player<'a>>,
}

#[derive(Debug)]
pub struct Galaxy<'a> {
    pub dimensions: (usize, usize),
    pub stars: Vec<Star<'a>>,
    pub star_matrix: na::SMatrix<usize, 100, 100>,
    pub distances: HashMap<usize, Vec<(usize, u32)>>,
    pub total_stars: usize,
}

impl<'a> Galaxy<'a> {
    pub fn new() -> Self {
        Galaxy {
            dimensions: (100 as usize, 100 as usize),
            stars: Vec::new(),
            star_matrix: na::SMatrix::<usize, 100, 100>::zeros(),
            distances: HashMap::new(),
            total_stars: 100 * 100 / 1000,
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

    fn add_star(&mut self, star: Star<'a>, index: usize) -> Result<(), &str> {
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
pub struct Star<'a> {
    pub name: String,
    pub planets: Vec<Planet<'a>>,
    pub location: (usize, usize),
}

impl<'a> Star<'a> {
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

impl<'a> PartialEq for Star<'a> {
    fn eq(&self, other: &Star<'a>) -> bool {
        self.name == other.name
    }
}

#[derive(Debug)]
pub struct UnitData<'a> {
    pub planet: &'a Planet<'a>,
    pub size: u32,
}

impl Display for UnitData<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UnitData {{ planet: {}, size: {} }}",
            self.planet.name, self.size
        )
    }
}

#[derive(Debug)]
pub struct Planet<'a> {
    pub name: String,
    pub units: Vec<&'a dyn Unit>,
}

pub trait Unit: fmt::Debug {}

#[derive(Debug)]
pub struct Player<'a> {
    pub name: String,
    pub start_planet: &'a Planet<'a>,
    pub controlled_planets: Vec<&'a Planet<'a>>,
    pub units: Vec<&'a dyn Unit>,
}

#[derive(Debug)]
pub struct Infrantry<'a> {
    pub data: UnitData<'a>,
}

#[derive(Debug)]
pub struct Laborers<'a> {
    pub data: UnitData<'a>,
}

impl Unit for Infrantry<'_> {}
impl Unit for Laborers<'_> {}
impl Unit for Ship<'_> {}

#[derive(Debug)]
enum ShipType {
    Cargo,
}

#[derive(Debug)]
struct Ship<'a> {
    pub data: UnitData<'a>,
    ship_type: ShipType,
}

impl<'a> Ship<'a> {
    fn new(location: &'a Planet, ship_type: ShipType) -> Self {
        match ship_type {
            ShipType::Cargo => Ship {
                ship_type: ship_type,
                data: UnitData {
                    planet: location,
                    size: 1,
                },
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut game = Game {
            galaxy: Galaxy::new(),
            players: Vec::new(),
            turn: 0,
        };
        let mut galaxy = game.galaxy;
        galaxy.populate_default();

        // Find the distance between two total_stars
        let star1 = &galaxy.stars[0];
        let star2 = &galaxy.stars[1];
        let distance = Star::distance_between(star1, star2);
        info!("Distances {:?}", galaxy.distances[&0]);

        // Init game with a laborers, infantry, and a cargo ship in the first star
        let laborers = Laborers {
            data: UnitData {
                planet: &galaxy.stars[0].planets[0],
                size: 1,
            },
        };
        let infrantry = Infrantry {
            data: UnitData {
                planet: &galaxy.stars[0].planets[0],
                size: 1,
            },
        };

        // Create a new Player
        let player = Player {
            name: "Player 1".to_string(),
            start_planet: &galaxy.stars[0].planets[0],
            controlled_planets: vec![&galaxy.stars[0].planets[0]],
            units: vec![&laborers, &infrantry],
        };
        info!("Player: {}, Units: {:?}", player.name, player.units);
        game.players.push(player);
    }
}
