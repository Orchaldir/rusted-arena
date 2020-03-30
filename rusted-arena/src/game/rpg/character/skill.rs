use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Skill {
    pub id: usize,
    pub name: String,
}

pub struct SkillMgr {
    name_map: HashMap<String, usize>,
    skill_map: HashMap<usize, Skill>,
}

impl SkillMgr {
    pub fn new(skills: Vec<Skill>) -> SkillMgr {
        let mut name_map = HashMap::new();
        let mut skill_map = HashMap::new();

        for skill in skills {
            name_map.insert(skill.name.clone(), skill.id);
            skill_map.insert(skill.id, skill);
        }

        SkillMgr {
            name_map,
            skill_map,
        }
    }

    pub fn get(&self, id: usize) -> &Skill {
        self.skill_map
            .get(&id)
            .unwrap_or_else(|| panic!("Skill with id {} is unknown!", id))
    }

    pub fn get_id(&self, name: &str) -> Option<usize> {
        self.name_map.get(name).copied()
    }
}

#[derive(Default)]
pub struct SkillBuilder {
    skills: Vec<Skill>,
}

impl SkillBuilder {
    pub fn create(mut self, name: String) -> SkillBuilder {
        let id = self.skills.len();
        self.skills.push(Skill { id, name });
        self
    }

    pub fn build(self) -> SkillMgr {
        SkillMgr::new(self.skills)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NAME_A: &'static str = "A";
    const NAME_B: &'static str = "B";
    const NAME_C: &'static str = "C";

    #[test]
    fn test_get() {
        let manager = SkillBuilder::default()
            .create(NAME_A.to_string())
            .create(NAME_B.to_string())
            .build();

        assert_skill(&manager, 0, NAME_A);
        assert_skill(&manager, 1, NAME_B);
    }

    #[test]
    #[should_panic(expected = "Skill with id 99 is unknown!")]
    fn test_get_unknown_skill() {
        let manager = SkillBuilder::default().build();

        manager.get(99);
    }

    #[test]
    fn test_get_id() {
        let manager = SkillBuilder::default()
            .create(NAME_A.to_string())
            .create(NAME_B.to_string())
            .build();

        assert_eq!(manager.get_id(NAME_A), Some(0));
        assert_eq!(manager.get_id(NAME_B), Some(1));
        assert_eq!(manager.get_id(NAME_C), None);
    }

    fn assert_skill(manager: &SkillMgr, id: usize, name: &str) {
        let skill = manager.get(id);
        assert_eq!(skill.id, id);
        assert_eq!(skill.name, name);
    }
}
