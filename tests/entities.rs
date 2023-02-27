use eyre::Result;
use stars::World;

#[test]
fn create_entity() -> Result<()> {
    let mut world = World::new();
    world.register_component::<Location>();
    world.register_component::<Size>();

    world
        .create_entity()
        .with(Location(0.0, 0.0))?
        .with(Size(3.0))?;
    Ok(())
}

#[test]
fn query_for_entities() -> Result<()> {
    let mut world = World::new();
    world.register_component::<Location>();
    world.register_component::<Size>();
    world
        .create_entity()
        .with(Location(9.0, 0.0))?
        .with(Size(3.0))?;

    world.create_entity().with(Location(0.0, 0.0))?;
    world.create_entity().with(Size(4.0))?;
    world
        .create_entity()
        .with(Location(1.0, 2.0))?
        .with(Size(5.0))?;

    let query = world.query().with::<Location>()?.with::<Size>()?.run();
    let locations = &query[0];
    let sizes = &query[1];
    assert_eq!(locations.len(), sizes.len());
    assert_eq!(locations.len(), 2);

    let borrowed_first_location = locations[0].borrow();
    let first_location = borrowed_first_location.downcast_ref::<Location>().unwrap();
    assert_eq!(first_location.0, 9.0);

    let borrowed_first_size = sizes[0].borrow();
    let first_size = borrowed_first_size.downcast_ref::<Size>().unwrap();
    assert_eq!(first_size.0, 3.0);

    Ok(())
}

struct Location(pub f32, pub f32);
struct Size(pub f32);
