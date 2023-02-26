mod resource;

use std::any::Any;

use resource::Resource;


#[derive(Default)]
pub struct World {
    resources: Resource,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_resource(&mut self, resource: impl Any) {
        self.resources.add(resource);
    }

    pub fn get_resource<T: Any>(&self) -> Option<&T> {
        self.resources.get_ref::<T>()
    }

    pub fn get_resource_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.resources.get_mut::<T>()
    }

    pub fn remove_resource<T: Any>(&mut self) {
        self.resources.remove::<T>();
    }
}