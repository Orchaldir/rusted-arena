pub mod component;
pub mod storage;
pub mod system;
pub mod testing;

use crate::utils::ecs::component::Component;
use crate::utils::ecs::storage::manager::StorageMgr;
use crate::utils::ecs::storage::ComponentStorage;
use std::any::{Any, TypeId};
use std::collections::HashMap;

#[derive(Default)]
pub struct ECS {
    entities: Vec<usize>,
    next_entity: usize,
    storage_mgr: StorageMgr,
    data_map: HashMap<TypeId, Box<dyn Any>>,
}

impl ECS {
    pub fn new() -> ECS {
        ECS {
            entities: Vec::new(),
            next_entity: 0,
            storage_mgr: StorageMgr::new(),
            data_map: HashMap::new(),
        }
    }

    pub fn create_entity(&mut self) -> EntityBuilder {
        let entity = self.next_entity;

        self.entities.push(entity);
        self.next_entity += 1;

        EntityBuilder {
            entity,
            storage_mgr: &mut self.storage_mgr,
        }
    }

    pub fn remove_entity(&mut self, entity: usize) {
        self.entities.retain(|e| *e != entity);
    }

    pub fn get_entities(&self) -> &[usize] {
        &self.entities
    }

    pub fn get_entities_of_2<A: Component, B: Component>(&self) -> Vec<usize> {
        let mut entities = self.entities.clone();
        let storage_a: &A::Storage = self.storage_mgr.get::<A>();
        let storage_b: &B::Storage = self.storage_mgr.get::<B>();

        storage_a.filter(&mut entities);
        storage_b.filter(&mut entities);

        entities
    }

    pub fn get_storage_mgr(&self) -> &StorageMgr {
        &self.storage_mgr
    }

    pub fn get_storage_mgr_mut(&mut self) -> &mut StorageMgr {
        &mut self.storage_mgr
    }

    // components

    pub fn unwrap_component<C: Component>(&self, entity: usize) -> &C {
        self.storage_mgr.get::<C>().get(entity).unwrap_or_else(|| {
            panic!(
                "Entity {} has no component of type '{}'!",
                entity,
                C::get_component_type()
            )
        })
    }

    pub fn unwrap_component_mut<C: Component>(&mut self, entity: usize) -> &mut C {
        self.storage_mgr
            .get_mut::<C>()
            .get_mut(entity)
            .unwrap_or_else(|| {
                panic!(
                    "Entity {} has no component of type '{}'!",
                    entity,
                    C::get_component_type()
                )
            })
    }

    // data

    pub fn put<T>(&mut self, data: T)
    where
        T: Any,
    {
        let type_id = TypeId::of::<T>();

        self.data_map.insert(type_id, Box::new(data));
    }

    pub fn get<T>(&self) -> &T
    where
        T: Any,
    {
        let type_id = TypeId::of::<T>();

        match self.data_map.get(&type_id) {
            Some(probably_data) => match probably_data.downcast_ref::<T>() {
                Some(data) => data,
                None => unreachable!(),
            },
            None => unreachable!(),
        }
    }
}

pub struct EntityBuilder<'a> {
    entity: usize,
    storage_mgr: &'a mut StorageMgr,
}

impl<'a> EntityBuilder<'a> {
    pub fn get_entity(&self) -> usize {
        self.entity
    }

    pub fn with<C: Component>(self, component: C) -> Self {
        let storage: &mut C::Storage = self.storage_mgr.get_mut::<C>();
        storage.add(self.entity, component);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::ecs::testing::*;

    const A: ComponentA = ComponentA { value: 10 };
    const B: ComponentB = ComponentB { value: 20 };

    #[test]
    fn test_create_entity_with_components() {
        let mut ecs = ECS::new();

        ecs.get_storage_mgr_mut().register::<ComponentA>();
        ecs.get_storage_mgr_mut().register::<ComponentB>();

        let entity = ecs.create_entity().with(A).with(B).get_entity();

        assert_eq!(
            ecs.get_storage_mgr().get::<ComponentA>().get(entity),
            Some(&A)
        );
        assert_eq!(
            ecs.get_storage_mgr().get::<ComponentB>().get(entity),
            Some(&B)
        );
    }

    #[test]
    #[should_panic(expected = "Component of type 'B' is not registered!")]
    fn test_add_not_registered_components() {
        let mut ecs = ECS::new();

        ecs.create_entity().with(B);
    }

    #[test]
    fn test_id_increases() {
        let mut ecs = ECS::new();

        assert_eq!(ecs.create_entity().get_entity(), 0);
        assert_eq!(ecs.create_entity().get_entity(), 1);
        assert_eq!(ecs.create_entity().get_entity(), 2);
    }

    #[test]
    fn test_get_entities() {
        let mut ecs = ECS::new();

        ecs.create_entity().get_entity();
        ecs.create_entity().get_entity();
        ecs.create_entity().get_entity();

        assert_eq!(ecs.get_entities(), &[0, 1, 2]);
    }

    #[test]
    fn test_get_entities_of_2() {
        let mut ecs = ECS::new();

        ecs.get_storage_mgr_mut().register::<ComponentA>();
        ecs.get_storage_mgr_mut().register::<ComponentB>();

        ecs.create_entity().with(A).get_entity();
        ecs.create_entity().with(B).get_entity();
        ecs.create_entity().with(A).with(B).get_entity();

        assert_eq!(ecs.get_entities_of_2::<ComponentA, ComponentB>(), vec![2]);
    }

    #[test]
    fn test_remove_entity() {
        let mut ecs = ECS::new();

        ecs.create_entity().get_entity();
        ecs.create_entity().get_entity();
        ecs.create_entity().get_entity();
        ecs.remove_entity(1);

        assert_eq!(ecs.get_entities(), &[0, 2]);
    }

    // component

    #[test]
    fn test_unwrap_component() {
        let mut ecs = ECS::new();

        ecs.get_storage_mgr_mut().register::<ComponentA>();

        let entity = ecs.create_entity().with(A).get_entity();

        assert_eq!(ecs.unwrap_component::<ComponentA>(entity), &A);
    }

    #[test]
    #[should_panic(expected = "Entity 0 has no component of type 'A'!")]
    fn test_unwrap_non_existing_component() {
        let mut ecs = ECS::new();

        ecs.get_storage_mgr_mut().register::<ComponentA>();

        ecs.unwrap_component::<ComponentA>(0);
    }

    #[test]
    fn test_unwrap_component_mut() {
        let mut ecs = ECS::new();

        ecs.get_storage_mgr_mut().register::<ComponentA>();

        let entity = ecs.create_entity().with(A).get_entity();

        ecs.unwrap_component_mut::<ComponentA>(entity).value += 2;

        assert_eq!(ecs.unwrap_component::<ComponentA>(entity).value, 12);
    }

    #[test]
    #[should_panic(expected = "Entity 2 has no component of type 'B'!")]
    fn test_unwrap_non_existing_component_mut() {
        let mut ecs = ECS::new();

        ecs.get_storage_mgr_mut().register::<ComponentB>();

        ecs.unwrap_component_mut::<ComponentB>(2);
    }

    // data

    #[test]
    fn test_put_get_data() {
        let data: u32 = 56;
        let mut ecs = ECS::new();

        ecs.put(data);

        assert_eq!(ecs.get::<u32>(), &data);
    }

    #[test]
    #[should_panic]
    fn test_get_no_data() {
        let ecs = ECS::new();

        ecs.get::<u32>();
    }
}
