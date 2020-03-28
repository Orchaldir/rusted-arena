use crate::utils::ecs::component::Component;
use crate::utils::ecs::storage::ComponentMap;
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComponentA {
    pub value: u32,
}

impl Component for ComponentA {
    type Storage = ComponentMap<Self>;

    fn get_component_type() -> &'static str {
        "A"
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComponentB {
    pub value: u32,
}

impl Component for ComponentB {
    type Storage = ComponentMap<Self>;

    fn get_component_type() -> &'static str {
        "B"
    }
}
