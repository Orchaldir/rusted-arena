use crate::utils::ecs::component::Component;
use crate::utils::ecs::storage::ComponentMap;

#[derive(Clone, Debug, PartialEq)]
pub enum HealthState {
    Healthy,
    Reeling,
    Unconsciousness,
    Dead,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Health {
    state: HealthState,
    penalty: u32,
}

impl Component for Health {
    type Storage = ComponentMap<Self>;

    fn get_component_type() -> &'static str {
        "Health"
    }
}

impl Default for Health {
    fn default() -> Health {
        Health {
            state: HealthState::Healthy,
            penalty: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::component::health::HealthState::Healthy;

    #[test]
    fn test_get_component_type() {
        assert_eq!(Health::get_component_type(), "Health");
    }

    #[test]
    fn test_default() {
        let health = Health::default();

        assert_eq!(health.state, Healthy);
        assert_eq!(health.penalty, 0);
    }
}
