extern crate nalgebra as na;
use crate::utils;
use memmap::Mmap;
use rand::prelude::*;
use std::fmt;
use std::fs;
use std::io::{Error, Write};
use std::{collections::HashMap, fmt::Display};

pub struct Game<'a> {
    galaxy: Galaxy<'a>,
    turn: u32,
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        let mut galaxy = Galaxy::new();
        galaxy.populate_default();
        Game {
            galaxy: galaxy,
            turn: 0,
        }
    }
    fn test(&self) {}
}

#[derive(Debug)]
struct Galaxy<'a> {
    dimensions: (usize, usize),
    stars: Vec<Star<'a>>,
    star_matrix: na::SMatrix<usize, 100, 100>,
    distances: HashMap<usize, Vec<(usize, u32)>>,
    total_stars: usize,
    players: Vec<Player<'a>>,
}

impl<'a> Galaxy<'a> {
    fn new() -> Self {
        Galaxy {
            dimensions: (100 as usize, 100 as usize),
            stars: Vec::new(),
            star_matrix: na::SMatrix::<usize, 100, 100>::zeros(),
            distances: HashMap::new(),
            total_stars: 100 * 100 / 1000,
            players: Vec::new(),
        }
    }

    fn add_star(&mut self, star: Star<'a>, index: usize) -> Result<(), &str> {
        // Check if the star is already in the matrix
        // If it is, throw an error
        if self.star_matrix[(star.location.0, star.location.1)] != 0 {
            println!(
                "Star already exists at location ({}, {})",
                star.location.0, star.location.1
            );
            return Err("Star already exists at location");
        }
        self.star_matrix[(star.location.0, star.location.1)] = index;
        println!(
            "Star {} added at location ({}, {})",
            star.name, star.location.0, star.location.1
        );
        self.stars.push(star);
        Ok(())
    }

    fn get_star_index(&self, star: &Star) -> usize {
        self.star_matrix[(star.location.0, star.location.1)]
    }

    fn populate_default(&mut self) {
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
                    println!("Planet {} added to star {}", planet.name, star.name);
                    star.planets.push(planet);
                }
                valid_location = match self.add_star(star, i) {
                    Ok(_) => true,
                    Err(_) => false,
                };
            }
        }
        self.calculcate_all_distances();
        println!("Galaxy populated with {} stars", self.total_stars);
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
struct Star<'a> {
    name: String,
    planets: Vec<Planet<'a>>,
    location: (usize, usize),
}

impl<'a> Star<'a> {
    fn distance_between(star1: &Star, star2: &Star) -> u32 {
        let x1 = star1.location.0 as f32;
        let y1 = star1.location.1 as f32;
        let x2 = star2.location.0 as f32;
        let y2 = star2.location.1 as f32;
        let distance = ((x1 - y1).powi(2) + (x2 - y2).powi(2)).sqrt() as u32;
        println!(
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
struct UnitData<'a> {
    planet: &'a Planet<'a>,
    size: u32,
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
struct Planet<'a> {
    name: String,
    units: Vec<&'a dyn Unit>,
}

trait Unit: fmt::Debug {}

#[derive(Debug)]
struct Player<'a> {
    name: String,
    start_planet: &'a Planet<'a>,
    controlled_planets: Vec<&'a Planet<'a>>,
    units: Vec<&'a dyn Unit>,
}

#[derive(Debug)]
struct Infrantry<'a> {
    data: UnitData<'a>,
}

#[derive(Debug)]
struct Laborers<'a> {
    data: UnitData<'a>,
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
    data: UnitData<'a>,
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
        let mut galaxy = Galaxy::new();
        galaxy.populate_default();

        // Find the distance between two total_stars
        let star1 = &galaxy.stars[0];
        let star2 = &galaxy.stars[1];
        let distance = Star::distance_between(star1, star2);
        println!("Distances {:?}", galaxy.distances[&0]);

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
        println!("Player: {}, Units: {:?}", player.name, player.units);
        galaxy.players.push(player);
    }
}
