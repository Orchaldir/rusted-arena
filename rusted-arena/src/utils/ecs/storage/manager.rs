use crate::utils::ecs::component::Component;
use crate::utils::ecs::storage::ComponentStorage;
use std::any::{Any, TypeId};
use std::collections::HashMap;

#[derive(Default)]
pub struct StorageMgr {
    storage_map: HashMap<TypeId, Box<dyn Any>>,
}

impl StorageMgr {
    pub fn new() -> Self {
        Self {
            storage_map: HashMap::new(),
        }
    }

    pub fn register<C: Component>(&mut self) {
        let type_id = TypeId::of::<C>();

        if self.storage_map.contains_key(&type_id) {
            panic!(
                "Component '{}' is already registered!",
                C::get_component_type()
            )
        }

        let new_storage = <C as Component>::Storage::new();
        self.storage_map.insert(type_id, Box::new(new_storage));
    }

    pub fn get<C: Component>(&self) -> &<C as Component>::Storage {
        let type_id = TypeId::of::<C>();

        match self.storage_map.get(&type_id) {
            Some(probably_storage) => {
                match probably_storage.downcast_ref::<<C as Component>::Storage>() {
                    Some(storage) => storage,
                    None => unreachable!(),
                }
            }
            None => panic!("Component '{}' is not registered!", C::get_component_type()),
        }
    }

    pub fn get_mut<C: Component>(&mut self) -> &mut <C as Component>::Storage {
        let type_id = TypeId::of::<C>();

        match self.storage_map.get_mut(&type_id) {
            Some(probably_storage) => {
                match probably_storage.downcast_mut::<<C as Component>::Storage>() {
                    Some(storage) => storage,
                    None => unreachable!(),
                }
            }
            None => panic!("Component '{}' is not registered!", C::get_component_type()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::ecs::testing::ComponentA;

    #[test]
    fn test_register_and_get() {
        let mut manager = StorageMgr::new();

        manager.register::<ComponentA>();

        let storage = manager.get::<ComponentA>();

        assert_eq!(storage.get(0), None);
    }

    #[test]
    #[should_panic(expected = "Component 'A' is already registered!")]
    fn test_register_twice() {
        let mut manager = StorageMgr::new();

        manager.register::<ComponentA>();
        manager.register::<ComponentA>();
    }

    #[test]
    #[should_panic(expected = "Component 'A' is not registered!")]
    fn test_get_without_register() {
        let manager = StorageMgr::new();

        manager.get::<ComponentA>();
    }

    #[test]
    fn test_register_and_get_mut() {
        let mut manager = StorageMgr::new();

        manager.register::<ComponentA>();

        let storage = manager.get_mut::<ComponentA>();

        storage.add(0, ComponentA { value: 5 });
    }

    #[test]
    #[should_panic(expected = "Component 'A' is not registered!")]
    fn test_get_mut_without_register() {
        let mut manager = StorageMgr::new();

        manager.get_mut::<ComponentA>();
    }
}
