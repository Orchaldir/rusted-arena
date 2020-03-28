use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;

trait Component: Debug + Sized + Any {
    type Storage: ComponentStorage<Self>;
}

pub trait ComponentStorage<T> {
    fn new() -> Self;
    fn add(&mut self, entity: usize, component: T);
    fn get(&self, entity: usize) -> Option<&T>;
    fn get_mut(&mut self, entity: usize) -> Option<&mut T>;
    fn remove(&mut self, entity: usize) -> Option<T>;
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct TestComponent {
        pub value: u32,
    }

    impl Component for TestComponent {
        type Storage = ComponentMap<Self>;
    }

    const ENTITY: usize = 42;
    const COMPONENT: TestComponent = TestComponent { value: 6 };

    #[test]
    fn test_add_and_get() {
        let mut storage: ComponentMap<TestComponent> = <TestComponent as Component>::Storage::new();

        storage.add(ENTITY, COMPONENT);

        assert_eq!(storage.get(ENTITY), Some(&COMPONENT));
    }

    #[test]
    fn test_add_and_get_mut() {
        let mut storage: ComponentMap<TestComponent> = <TestComponent as Component>::Storage::new();

        storage.add(ENTITY, COMPONENT);

        if let Some(component) = storage.get_mut(ENTITY) {
            component.value = 12;
        }

        assert_eq!(storage.get(ENTITY), Some(&TestComponent { value: 12 }));
    }

    #[test]
    fn test_get_unknown_entity() {
        let mut storage: ComponentMap<TestComponent> = <TestComponent as Component>::Storage::new();

        assert_eq!(storage.get(ENTITY), None);
        assert_eq!(storage.get_mut(ENTITY), None);
    }

    #[test]
    fn test_remove() {
        let mut storage: ComponentMap<TestComponent> = <TestComponent as Component>::Storage::new();

        storage.add(ENTITY, COMPONENT);

        assert_eq!(storage.remove(ENTITY), Some(COMPONENT));
        assert_eq!(storage.get(ENTITY), None);
    }
}
