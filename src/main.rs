extern crate nalgebra as na;
use rand::prelude::*;
use std::collections::HashMap;

struct Galaxy {
    dimensions: (usize, usize),
    stars: Vec<Star>,
    star_matrix: na::SMatrix<usize, 100, 100>,
    distances: HashMap<usize, Vec<(usize, f32)>>,
    total_stars: usize,
}

impl Galaxy {
    fn new() -> Self {
        Galaxy {
            dimensions: (100 as usize, 100 as usize),
            stars: Vec::new(),
            star_matrix: na::SMatrix::<usize, 100, 100>::zeros(),
            distances: HashMap::new(),
            total_stars: 100 * 100 / 10,
        }
    }

    fn add_star(&mut self, star: Star, index: usize) -> Result<(), &str> {
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
                let star = Star {
                    name: String::from("Sol"),
                    planets: Vec::new(),
                    location: (x, y),
                };
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
                let distance = Star::distance_between(star, other_star);
                let star_and_distance = (self.get_star_index(other_star), distance);
                distances_to.push(star_and_distance);
            }
            distances_to.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            self.distances.insert(i, distances_to);
        }
    }
}

struct Star {
    name: String,
    planets: Vec<Planet>,
    location: (usize, usize),
}

impl Star {
    fn distance_between(star1: &Star, star2: &Star) -> f32 {
        let x1 = star1.location.0 as f32;
        let y1 = star1.location.1 as f32;
        let x2 = star2.location.0 as f32;
        let y2 = star2.location.1 as f32;
        let distance = ((x1 - y1).powi(2) + (x2 - y2).powi(2)).sqrt();
        println!(
            "Distance between {} and {} is {}",
            star1.name, star2.name, distance
        );
        distance
    }
}

struct Planet {}

fn main() {
    // Create a new galaxy
    let mut galaxy = Galaxy::new();
    galaxy.populate_default();

    // Find the distance between two total_stars
    let star1 = &galaxy.stars[0];
    let star2 = &galaxy.stars[1];
    let distance = Star::distance_between(star1, star2);
    println!("Distances {:?}", galaxy.distances[&0]);
}
