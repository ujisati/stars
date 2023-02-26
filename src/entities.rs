use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

#[derive(Debug, Default)]
pub struct Entities {
    components: HashMap<TypeId, Vec<Option<Rc<RefCell<dyn Any>>>>>,
}

impl Entities {
    pub fn register_component<T: Any + 'static>(&mut self) {
        self.components.insert(TypeId::of::<T>(), vec![]);
    }

    pub fn create_entity(&mut self) {
        self.components.iter_mut().for_each(|(_, components)| {
            components.push(None);
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn register_entity() {
        let mut entities = Entities::default();
        entities.register_component::<Health>();
        let type_id = TypeId::of::<Health>();
        let health_componenets = entities.components.get(&type_id).unwrap();
        assert_eq!(health_componenets.len(), 0);
    }

    #[test]
    fn create_entity() {
        let mut entities = Entities::default();
        entities.register_component::<Health>();
        entities.register_component::<Speed>();
        entities.create_entity();
        let health = entities.components.get(&TypeId::of::<Health>()).unwrap();
        let speed = entities.components.get(&TypeId::of::<Speed>()).unwrap();
        assert!(health.len() == speed.len() && health.len() == 1);
        assert!(health[0].is_none() && speed[0].is_none());
    }

    struct Health(pub u32);
    struct Speed(pub u32);
}
