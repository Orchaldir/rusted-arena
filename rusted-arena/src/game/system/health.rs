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
    pub fn take_damage(&self, ecs: &mut ECS, target: usize, damage: &Damage) {
        let toughness_rank = self.get_toughness(ecs, target);
        let health = ecs.unwrap_component_mut::<Health>(target);
        let difficulty = toughness_rank - health.penalty as i32;

        match self.checker.check(damage.rank, difficulty) {
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
    use crate::game::component::health::HealthState;
    use crate::game::component::health::HealthState::*;
    use crate::game::component::stats::StatsBuilder;
    use crate::game::rpg::character::skill::Skill;
    use crate::game::rpg::check::*;

    #[test]
    fn test_take_damage_failure() {
        test_take_damage(CheckResult::Failure(1), 6, 4, Healthy, 1)
    }

    #[test]
    fn test_take_damage_success() {
        test_take_damage(CheckResult::Success(1), 6, 4, Reeling, 0)
    }

    fn test_take_damage(
        check_result: CheckResult,
        toughness_rank: i32,
        damage_rank: i32,
        result_state: HealthState,
        result_penalty: u32,
    ) {
        let mut mock = MockChecker::new();
        mock.expect_check().return_const(check_result);

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
            .with(
                StatsBuilder::default()
                    .add_skill(&toughness, toughness_rank)
                    .build(),
            )
            .get_entity();

        let damage = Damage { rank: damage_rank };

        let system = HealthSystem {
            checker: &mock,
            toughness: &toughness,
        };

        system.take_damage(&mut ecs, entity, &damage);

        let health = ecs.unwrap_component::<Health>(entity);

        assert_eq!(health.state, result_state);
        assert_eq!(health.penalty, result_penalty);
    }
}
