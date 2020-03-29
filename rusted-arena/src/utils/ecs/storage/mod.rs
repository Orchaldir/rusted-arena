pub mod manager;

use std::collections::HashMap;

pub trait ComponentStorage<T> {
    fn new() -> Self;
    fn add(&mut self, entity: usize, component: T);
    fn get(&self, entity: usize) -> Option<&T>;
    fn get_mut(&mut self, entity: usize) -> Option<&mut T>;
    fn remove(&mut self, entity: usize) -> Option<T>;
    fn filter(&self, entities: &mut Vec<usize>);
}

pub struct ComponentMap<T> {
    components: HashMap<usize, T>,
}

impl<T> ComponentStorage<T> for ComponentMap<T> {
    fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    fn add(&mut self, entity: usize, component: T) {
        self.components.insert(entity, component);
    }

    fn get(&self, entity: usize) -> Option<&T> {
        self.components.get(&entity)
    }

    fn get_mut(&mut self, entity: usize) -> Option<&mut T> {
        self.components.get_mut(&entity)
    }

    fn remove(&mut self, entity: usize) -> Option<T> {
        self.components.remove(&entity)
    }

    fn filter(&self, entities: &mut Vec<usize>) {
        entities.retain(|e| self.components.contains_key(e));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::ecs::component::Component;
    use crate::utils::ecs::testing::ComponentA;

    const ENTITY: usize = 42;
    const COMPONENT: ComponentA = ComponentA { value: 6 };

    #[test]
    fn test_add_and_get() {
        let mut storage: ComponentMap<ComponentA> = <ComponentA as Component>::Storage::new();

        storage.add(ENTITY, COMPONENT);

        assert_eq!(storage.get(ENTITY), Some(&COMPONENT));
    }

    #[test]
    fn test_add_and_get_mut() {
        let mut storage: ComponentMap<ComponentA> = <ComponentA as Component>::Storage::new();

        storage.add(ENTITY, COMPONENT);

        if let Some(component) = storage.get_mut(ENTITY) {
            component.value = 12;
        }

        assert_eq!(storage.get(ENTITY), Some(&ComponentA { value: 12 }));
    }

    #[test]
    fn test_get_unknown_entity() {
        let mut storage: ComponentMap<ComponentA> = <ComponentA as Component>::Storage::new();

        assert_eq!(storage.get(ENTITY), None);
        assert_eq!(storage.get_mut(ENTITY), None);
    }

    #[test]
    fn test_remove() {
        let mut storage: ComponentMap<ComponentA> = <ComponentA as Component>::Storage::new();

        storage.add(ENTITY, COMPONENT);

        assert_eq!(storage.remove(ENTITY), Some(COMPONENT));
        assert_eq!(storage.get(ENTITY), None);
    }

    #[test]
    fn test_filter() {
        let mut storage: ComponentMap<ComponentA> = <ComponentA as Component>::Storage::new();
        let mut entities = vec![0, ENTITY, 100];

        storage.add(ENTITY, COMPONENT);
        storage.filter(&mut entities);

        assert_eq!(entities, vec![ENTITY]);
    }
}
