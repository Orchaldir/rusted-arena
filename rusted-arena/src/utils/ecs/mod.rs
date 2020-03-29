pub mod component;
pub mod storage;
pub mod testing;

use crate::utils::ecs::component::Component;
use crate::utils::ecs::storage::manager::StorageMgr;
use crate::utils::ecs::storage::ComponentStorage;

#[derive(Default)]
pub struct ECS {
    entities: Vec<usize>,
    next_entity: usize,
    storage_mgr: StorageMgr,
}

impl ECS {
    pub fn new() -> ECS {
        ECS {
            entities: Vec::new(),
            next_entity: 0,
            storage_mgr: StorageMgr::new(),
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

    pub fn get_entities_of_2<A: Component, B: Component>(&self) -> Vec<usize>{
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
    #[should_panic(expected = "Component 'B' is not registered!")]
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
}
