use crate::utils::ecs::component::Component;
use crate::utils::ecs::storage::ComponentMap;

#[derive(Clone, Debug, PartialEq)]
pub enum HealthState {
    Healthy,
    Reeling,
    Unconsciousness,
    Dead,
}

impl HealthState {
    pub fn get_worse(&self) -> HealthState {
        match self {
            HealthState::Healthy => HealthState::Reeling,
            HealthState::Reeling => HealthState::Unconsciousness,
            HealthState::Unconsciousness => HealthState::Dead,
            HealthState::Dead => HealthState::Dead,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Health {
    pub state: HealthState,
    pub penalty: u32,
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
    use crate::game::component::health::HealthState::*;

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

    #[test]
    fn test_get_worse() {
        assert_eq!(Healthy.get_worse(), Reeling);
        assert_eq!(Reeling.get_worse(), Unconsciousness);
        assert_eq!(Unconsciousness.get_worse(), Dead);
        assert_eq!(Dead.get_worse(), Dead);
    }
}
