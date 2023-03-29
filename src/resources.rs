use bevy::prelude::*;
use log;
use rand::Rng;

#[derive(Resource)]
pub struct Config {
    pub galaxy_dimension: u32,
    pub num_stars: u32,
}

impl Config {
    pub fn validate(self) -> Self {
        if self.num_stars > self.galaxy_dimension.pow(2) {
            panic!("num_stars must be less than galaxy_dimension^2");
        }
        self
    }
}

impl FromWorld for Config {
    fn from_world(world: &mut World) -> Self {
        log::info!("creating config");
        Config {
            galaxy_dimension: 25,
            num_stars: 100,
        }
        .validate()
    }
}
