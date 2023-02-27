use std::{any::Any, cell::RefCell, rc::Rc};

pub struct Query {}

impl Query {
    pub fn new() -> Self {
        Self {}
    }

    pub fn with<T: Any>(&mut self) -> &mut Self {
        self
    }

    pub fn run(&self) -> Vec<Vec<Rc<RefCell<dyn Any>>>> {
        vec![]
    }

}