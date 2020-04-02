use crate::utils::ecs::ECS;

pub trait System<A> {
    fn handle(&mut self, ecs: &mut ECS, action: &A);
}

pub struct SystemMgr<A> {
    systems: Vec<Box<dyn System<A>>>,
}

impl<A> SystemMgr<A> {
    pub fn handle(&mut self, ecs: &mut ECS, action: &A) {
        for system in &mut self.systems {
            system.handle(ecs, &action);
        }
    }
}
