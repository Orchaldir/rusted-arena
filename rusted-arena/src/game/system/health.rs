use crate::game::component::health::Health;
use crate::game::component::stats::Stats;
use crate::game::rpg::character::skill::Skill;
use crate::game::rpg::check::{CheckResult, Checker};
use crate::game::rpg::combat::damage::Damage;
use crate::utils::ecs::storage::ComponentStorage;
use crate::utils::ecs::ECS;

pub struct HealthSystem<'a> {
    checker: &'a dyn Checker,
    toughness: &'a Skill,
}

impl<'a> HealthSystem<'a> {
    pub fn take_damage(&mut self, ecs: &mut ECS, target: usize, damage: &Damage) {
        let toughness_rank = self.get_toughness(ecs, target);
        let health = ecs
            .get_storage_mgr_mut()
            .get_mut::<Health>()
            .get_mut(target)
            .unwrap_or_else(|| panic!("Entity {} has no Health component!", target));

        let rank = toughness_rank - health.penalty as i32;

        match self.checker.check(rank, damage.rank) {
            CheckResult::Success(_) => {
                health.state = health.state.get_worse();
            }
            CheckResult::Failure(_) => {
                health.penalty += 1;
            }
        }
    }

    fn get_toughness(&self, ecs: &ECS, target: usize) -> i32 {
        let stats = ecs
            .get_storage_mgr()
            .get::<Stats>()
            .get(target)
            .unwrap_or_else(|| panic!("Entity {} has no Stats component!", target));
        stats
            .get_skill_rank(self.toughness)
            .expect("No default for skill Toughness!")
    }
}
