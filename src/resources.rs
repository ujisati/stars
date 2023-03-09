use bevy::prelude::*;
use rand::Rng;

#[derive(Resource)]
pub struct Galaxy {
    stars: Vec<Vec<Option<usize>>>,
}

impl FromWorld for Galaxy {
    fn from_world(_world: &mut World) -> Self {
        let galaxy_dim = 100; // TODO: read from config
        let mut stars = vec![vec![None; galaxy_dim]; galaxy_dim];
        let mut stars_id: usize = 0;
        for x in 0..galaxy_dim {
            for y in 0..galaxy_dim {
                // get a random number between 0 and 1
                // TODO: create interesting distributions
                let mut rng = rand::thread_rng();
                let star_present = rng.gen::<bool>();
                if !star_present {
                    stars[x][y] = None;
                    continue;
                }
                stars[x][y] = Some(stars_id);
                stars_id += 1;
            }
        }
        Galaxy { stars: stars }
    }
}
