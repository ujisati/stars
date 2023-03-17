use bevy::prelude::*;
use rand::Rng;
use log;

#[derive(Resource)]
pub struct Galaxy {
    pub object_grid: Vec<Vec<Option<usize>>>,
}

#[derive(Resource)]
pub struct Config {
    pub galaxy_dim: usize,
}

impl FromWorld for Config {
    fn from_world(world: &mut World) -> Self {
        log::info!("creating config");
        Config { galaxy_dim: 25 }
    }
}

impl FromWorld for Galaxy {
    fn from_world(world: &mut World) -> Self {
        log::info!("creating galaxy");
        let config = world.get_resource::<Config>().expect("config not found");
        let galaxy_dim = config.galaxy_dim;
        let mut stars = vec![vec![None; galaxy_dim]; galaxy_dim];
        let mut stars_id: usize = 0;
        for x in 0..galaxy_dim {
            for y in 0..galaxy_dim {
                log::trace!("creating galactic object at {}, {}", x, y);
                // get a random number between 0 and 1
                // TODO: create interesting distributions
                let mut rng = rand::thread_rng();
                let star_present = rng.gen::<bool>();
                if !star_present {
                    log::trace!("no star at {}, {}", x, y);
                    stars[x][y] = None;
                    continue;
                }
                stars[x][y] = Some(stars_id);
                stars_id += 1;
            }
        }
        log::info!("created galaxy");
        Galaxy { object_grid: stars }
    }
}
