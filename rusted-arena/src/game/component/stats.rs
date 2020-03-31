use crate::game::rpg::character::skill::Skill;
use crate::utils::ecs::component::Component;
use crate::utils::ecs::storage::ComponentMap;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Stats {
    skill_map: HashMap<usize, i32>,
}

impl Component for Stats {
    type Storage = ComponentMap<Self>;

    fn get_component_type() -> &'static str {
        "Stats"
    }
}

impl Stats {
    pub fn new(skill_map: HashMap<usize, i32>) -> Stats {
        Stats { skill_map }
    }

    pub fn get_skill_rank(&self, skill: &Skill) -> Option<i32> {
        let rank;

        if let Some(base) = self.skill_map.get(&skill.id) {
            rank = *base;
        } else if let Some(default) = skill.default {
            rank = default;
        } else {
            return None;
        }

        Some(rank)
    }
}

#[derive(Default)]
pub struct StatsBuilder {
    skill_map: HashMap<usize, i32>,
}

impl StatsBuilder {
    pub fn add_skill(mut self, skill: &Skill, rank: i32) -> StatsBuilder {
        self.skill_map.insert(skill.id, rank);
        self
    }

    pub fn build(self) -> Stats {
        Stats {
            skill_map: self.skill_map,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_component_type() {
        assert_eq!(Stats::get_component_type(), "Stats");
    }

    #[test]
    fn test_get_skill_rank() {
        let skill_a = create_a();
        let skill_b = create_b();
        let stats = StatsBuilder::default()
            .add_skill(&skill_a, 6)
            .add_skill(&skill_b, 7)
            .build();

        assert_eq!(stats.get_skill_rank(&skill_a), Some(6));
        assert_eq!(stats.get_skill_rank(&skill_b), Some(7));
    }

    #[test]
    fn test_get_skill_rank_default() {
        let skill_a = create_a();
        let skill_b = create_b();
        let stats = StatsBuilder::default().build();

        assert_eq!(stats.get_skill_rank(&skill_a), None);
        assert_eq!(stats.get_skill_rank(&skill_b), Some(-3));
    }

    fn create_a() -> Skill {
        Skill {
            id: 0,
            name: "A".to_string(),
            default: None,
        }
    }

    fn create_b() -> Skill {
        Skill {
            id: 1,
            name: "B".to_string(),
            default: Some(-3),
        }
    }
}
