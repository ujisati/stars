use eyre::Result;
use std::{
    any::{Any, TypeId},
    cell::RefCell,
    rc::Rc,
};

use super::Entities;
use super::Errors;

pub struct Query<'a> {
    map: u32,
    entities: &'a Entities,
    type_ids: Vec<TypeId>,
}

impl<'a> Query<'a> {
    pub fn new(entities: &'a Entities) -> Self {
        Self {
            entities,
            map: 0,
            type_ids: vec![],
        }
    }

    pub fn with<T: Any>(&mut self) -> Result<&mut Self> {
        let type_id = TypeId::of::<T>();
        if let Some(bit_mask) = self.entities.get_bitmask(&type_id) {
            self.map |= bit_mask;
            self.type_ids.push(type_id);
        } else {
            return Err(Errors::ComponentNotRegistered.into());
        }
        Ok(self)
    }

    pub fn run(&self) -> Vec<Vec<Rc<RefCell<dyn Any>>>> {
        let indexes: Vec<usize> = self
            .entities
            .map
            .iter()
            .enumerate()
            .filter_map(|(index, entity_map)| {
                if entity_map & self.map == self.map {
                    Some(index)
                } else {
                    None
                }
            })
            .collect();
        let mut result = vec![];
        for type_id in &self.type_ids {
            let entity_components = self.entities.components.get(type_id).unwrap();
            let mut components_to_keep = vec![];
            for index in &indexes {
               components_to_keep.push(entity_components[*index].as_ref().unwrap().clone());
            }
            result.push(components_to_keep);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn query_mask_updating_with_component() -> Result<()> {
        let mut entities = Entities::default();
        entities.register_component::<u32>();
        entities.register_component::<f32>();

        let mut query = Query::new(&entities);
        query.with::<u32>()?.with::<f32>()?;
        assert_eq!(query.map, 3);
        assert_eq!(TypeId::of::<u32>(), query.type_ids[0]);
        assert_eq!(TypeId::of::<f32>(), query.type_ids[1]);
        Ok(())
    }

    #[test]
    fn run_query() -> Result<()> {
        let mut entities = Entities::default();
        entities.register_component::<u32>();
        entities.register_component::<f32>();
        entities.create_entity().with(1u32)?.with(1.0f32)?;
        entities.create_entity().with(2u32)?;
        entities.create_entity().with(3.0f32)?;
        entities.create_entity().with(3u32)?.with(4.0f32)?;

        let mut query = Query::new(&entities);
        query.with::<u32>()?.with::<f32>()?;

        let query_result = query.run();
        let u32s = &query_result[0];
        let f32s = &query_result[1];

        assert_eq!(u32s.len(), f32s.len());
        assert_eq!(u32s.len(), 2);

        let borrowed_first_u32 = u32s[0].borrow();
        let first_u32 = borrowed_first_u32.downcast_ref::<u32>().unwrap();
        assert_eq!(*first_u32, 1);

        let borrowed_first_f32 = f32s[0].borrow();
        let first_f32 = borrowed_first_f32.downcast_ref::<f32>().unwrap();
        assert_eq!(*first_f32, 1.0);

        let borrowed_second_u32 = u32s[1].borrow();
        let second_u32 = borrowed_second_u32.downcast_ref::<u32>().unwrap();
        assert_eq!(*second_u32, 3);

        let borrowed_second_f32 = f32s[1].borrow();
        let second_f32 = borrowed_second_f32.downcast_ref::<f32>().unwrap();
        assert_eq!(*second_f32, 4.0);

        Ok(())
    }
}
