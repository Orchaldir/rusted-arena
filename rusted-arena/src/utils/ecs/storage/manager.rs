use crate::utils::ecs::storage::{Component, ComponentStorage};
use std::any::{Any, TypeId};
use std::collections::HashMap;

struct StorageMgr {
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
            panic!("Component {:?} is already registered!", type_id)
        }

        let new_storage = <C as Component>::Storage::new();
        self.storage_map.insert(type_id, Box::new(new_storage));
    }

    pub fn get<C: Component>(&mut self) -> &<C as Component>::Storage {
        let type_id = TypeId::of::<C>();

        match self.storage_map.get(&type_id) {
            Some(probably_storage) => {
                match probably_storage.downcast_ref::<<C as Component>::Storage>() {
                    Some(storage) => storage,
                    None => unreachable!(),
                }
            }
            None => panic!("Component {:?} is not registered!", type_id),
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
            None => panic!("Component {:?} is not registered!", type_id),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::ecs::storage::ComponentMap;

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct TestComponent {
        pub value: u32,
    }

    impl Component for TestComponent {
        type Storage = ComponentMap<Self>;
    }

    #[test]
    fn test_register_and_get() {
        let mut manager = StorageMgr::new();

        manager.register::<TestComponent>();

        let storage = manager.get::<TestComponent>();

        assert_eq!(storage.get(0), None);
    }

    #[test]
    #[should_panic]
    fn test_get_without_register() {
        let mut manager = StorageMgr::new();

        manager.get::<TestComponent>();
    }

    #[test]
    fn test_register_and_get_mut() {
        let mut manager = StorageMgr::new();

        manager.register::<TestComponent>();

        let storage = manager.get_mut::<TestComponent>();

        storage.add(0, TestComponent { value: 5 });
    }

    #[test]
    #[should_panic]
    fn test_get_mut_without_register() {
        let mut manager = StorageMgr::new();

        manager.get_mut::<TestComponent>();
    }
}
