use crate::tech;
use bevy::prelude::*;

enum Structure {
    Pipeline,
    ResearchLab,
    Factory,
    SpacePort,
    Radar,
    SignalJammer,
}


#[derive(Component)]
struct Location {
    star_x: usize,
    star_y: usize,
    planet_z: usize,
}

#[derive(Component)]
struct Player {
    name: String,
}

#[derive(Component)]
struct Visibility {
    /*
        Visibility can be jammed or certain ships could have stealth,
    */
    range: u32,
}

mod astronomy {
    enum AstronomicalObject {
        Star,
        Planet,
        Moon,
        BlackHole,
        Nebula,
        AsteroidBelt,
    }
}

mod structure {
    use super::*;

    #[derive(Component)]
    struct Size(u32);
}

mod ship {
    use super::*;

    #[derive(Component)] // TODO: does this need to be / should be a component?
    enum FuelEfficiency {
        Fossil = 1,
        Solar = 2,
        Deuterium = 3,
    }

    #[derive(Component)]
    enum DefenseSystem {
        Laser(u32),
    }

    #[derive(Component)]
    struct Engine {
        current_fuel: u32,
        max_fuel: u32,
        fuel_efficiency: FuelEfficiency,
    }

    #[derive(Component)]
    struct CargoBay {
        current_cargo: u32,
        max_cargo: u32,
    }

    #[derive(Bundle)]
    struct ShipBundle {
        location: Location,
        engine: Engine,
        cargo_bay: CargoBay,
        defense_system: DefenseSystem,
    }


    #[derive(Component)]
    struct Probe {
                  
    }
}
