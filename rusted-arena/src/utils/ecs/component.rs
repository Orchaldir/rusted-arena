use crate::utils::ecs::storage::ComponentStorage;
use std::any::Any;
use std::fmt::Debug;

pub trait Component: Debug + Sized + Any {
    type Storage: ComponentStorage<Self>;

    fn get_component_type() -> &'static str;
}
