use crate::game::component::health::Health;
use crate::game::component::stats::Stats;
use crate::game::rpg::character::skill::Skill;
use crate::game::rpg::check::{CheckResult, Checker};
use crate::game::rpg::combat::damage::Damage;
use crate::utils::ecs::ECS;

pub struct HealthSystem<'a> {
    checker: &'a dyn Checker,
    toughness: &'a Skill,
}

impl<'a> HealthSystem<'a> {
    pub fn take_damage(&mut self, ecs: &mut ECS, target: usize, damage: &Damage) {
        let toughness_rank = self.get_toughness(ecs, target);
        let health = ecs.unwrap_component_mut::<Health>(target);
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
        let stats = ecs.unwrap_component::<Stats>(target);
        stats
            .get_skill_rank(self.toughness)
            .expect("No default for skill Toughness!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::component::health::HealthState::*;
    use crate::game::component::stats::StatsBuilder;
    use crate::game::rpg::character::skill::Skill;
    use crate::game::rpg::check::*;

    #[test]
    fn test_take_no_damage() {
        let mut mock = MockChecker::new();
        mock.expect_check().return_const(CheckResult::Failure(1));

        let toughness = Skill {
            id: 0,
            name: "T".to_string(),
            default: None,
        };

        let mut ecs = ECS::new();

        ecs.get_storage_mgr_mut().register::<Health>();
        ecs.get_storage_mgr_mut().register::<Stats>();

        let entity = ecs
            .create_entity()
            .with(Health::default())
            .with(StatsBuilder::default().add_skill(&toughness, 6).build())
            .get_entity();

        let damage = Damage { rank: 4 };

        let mut system = HealthSystem {
            checker: &mock,
            toughness: &toughness,
        };

        system.take_damage(&mut ecs, entity, &damage);

        let health = ecs.unwrap_component::<Health>(entity);

        assert_eq!(health.state, Healthy);
        assert_eq!(health.penalty, 1);
    }
}
