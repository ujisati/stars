use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("attempting to add component to an entity without calling create component first")]
    CreateComponentNeverCalled,

    #[error("attempted to insert data for component that wasn't registered")]
    ComponentNotRegistered,
}