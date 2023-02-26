use stars::World;

#[test]
fn create_entity() {
    let mut world = World::new();
    world.register_component::<Location>();
    world.register_component::<Size>();

    world
        .create_entity()
        .with(Location(0.0, 0.0))
        .unwrap()
        .with(Size(3.0));
}

struct Location(pub f32, pub f32);
struct Size(pub f32);
