enum Size {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
}

struct Star {
    name: String,
}

struct Planet {}

struct Location(pub u32, pub u32);

// this is a resource
struct Galaxy {
    dimension: u32,
    grid: Vec<Vec<bool>>,
}

fn main() {
    println!("Hello, world!");
}
