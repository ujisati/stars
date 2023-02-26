use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

#[derive(Default)]
pub struct Resource {
    data: HashMap<TypeId, Box<dyn Any>>,
}

impl Resource {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, data: impl Any) {
        let type_id = data.type_id();
        self.data.insert(type_id, Box::new(data));
    }

    pub fn get_ref<T: Any>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        if let Some(data) = self.data.get(&type_id) {
            data.downcast_ref()
        } else {
            None
        }
    }

    pub fn get_mut<T: Any>(&mut self) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        if let Some(data) = self.data.get_mut(&type_id) {
            data.downcast_mut()
        } else {
            None
        }
    }

    pub fn remove<T: Any>(&mut self) {
        let type_id = TypeId::of::<T>();
        self.data.remove(&type_id);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_resource() {
        let resources = initialize_resource();

        let stored_resource = resources
            .data
            .get(&TypeId::of::<ExampleResource>())
            .unwrap();
        let extracted_resource = stored_resource.downcast_ref::<ExampleResource>().unwrap();
        assert_eq!(extracted_resource.0, 1);
    }

    #[test]
    fn get_resource() {
        let resources = initialize_resource();
        if let Some(extracted_resource) = resources.get_ref::<ExampleResource>() {
            assert_eq!(extracted_resource.0, 1);
        }
    }

    #[test]
    fn get_resource_mut() {
        let mut resources = initialize_resource();
        {
            let mut extracted_resource = resources.get_mut::<ExampleResource>().unwrap();
            extracted_resource.0 = 2;
        }
        let extracted_resource = resources.get_ref::<ExampleResource>().unwrap();
        assert_eq!(extracted_resource.0, 2);
    }


    #[test]
    fn remove_resource() {
        let mut resources = initialize_resource();
        resources.remove::<ExampleResource>();
        let example_type_id = TypeId::of::<ExampleResource>();
        assert!(resources.data.contains_key(&example_type_id) == false);
    }

    fn initialize_resource() -> Resource {
        let mut resources = Resource::new();
        let example_resource = ExampleResource(1);

        resources.add(example_resource);

        resources
    }

    struct ExampleResource(u32);
}
