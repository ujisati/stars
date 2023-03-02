use crate::tech;
use bevy::prelude::*;

enum Unit {
    Probe,
    CargoShip,
}

enum Structure {
    Pipeline,
    ResearchLab,
    Factory,
    SpacePort,
    Radar,
    SignalJammer,
}

struct Location {
    star_x: usize,
    star_y: usize,
    planet_z: usize,
}

struct Player {
    name: String,
}

struct Visibility {
    /*
        Visibility can be jammed or certain ships could have stealth,
        in which case the unit must keep track of its own visibility status
        but what if it can be seen by some things but not others?
    */
    range: u32,
    units_visible: Vec<Unit>,
    structures_visible: Vec<Structure>,
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
    #[derive(Component)]
    struct Size(u32);
}

mod ship {
    #[derive(Component)] // TODO: does this need to be / should be a component?
    enum FuelEfficiency {
        Fossil = 1,
        Solar = 2,
        Deuterium = 3,
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

    #[derive(Component)]
    enum DefenseSystem {
        Laser(u32),
    }
}
