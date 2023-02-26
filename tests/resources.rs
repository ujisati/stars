use stars::World;

#[test]
fn create_and_get_resource_immutably() {
    let mut world = initialize_world();

    let fps = world.get_resource::<ExampleResource>().unwrap();
    assert_eq!(fps.0, 1);
}

#[test]
fn get_resources_mutably() {
    let mut world = initialize_world();

    {
        let mut example_resource = world.get_resource_mut::<ExampleResource>().unwrap();
        example_resource.0 = 2;
    }
    let example_resource = world.get_resource::<ExampleResource>().unwrap();
    assert_eq!(example_resource.0, 2);
}

#[test]
fn delete_resource() {
    let mut world = initialize_world();

    world.remove_resource::<ExampleResource>();
    let deleted_resource = world.get_resource::<ExampleResource>();
    assert!(deleted_resource.is_none());
}


fn initialize_world() -> World {
    let mut world = World::new();

    world.add_resource(ExampleResource(1));
    world
}

struct ExampleResource(u32);
