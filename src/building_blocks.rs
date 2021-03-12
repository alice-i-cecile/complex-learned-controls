use bevy::ecs::{bundle::Bundle, entity::Entity};
use bevy::transform::components::Transform;
#[derive(Clone)]
pub struct Mass(pub u32);

impl Default for Mass {
    fn default() -> Self {
        Mass(1)
    }
}
#[derive(Default, Clone)]
pub struct Point;

#[derive(Bundle, Clone)]
pub struct PointBundle {
    pub point: Point,
    pub transform: Transform,
    pub mass: Mass,
}

impl PointBundle {
    pub fn new() -> Self {
        PointBundle {
            point: Default::default(),
            transform: Default::default(),
            mass: Default::default(),
        }
    }
}

#[derive(Default, Clone)]
pub struct Spring;
#[derive(Default, Clone)]
pub struct SpringStrength(pub u32);
#[derive(Bundle, Clone)]
pub struct SpringBundle {
    pub spring: Spring,
    pub transform: Transform,
    pub spring_strength: SpringStrength,
    pub connected: (Entity, Entity),
}

impl SpringBundle {
    pub fn new(connected_masses: (Entity, Entity)) -> Self {
        SpringBundle {
            spring: Default::default(),
            transform: Default::default(),
            spring_strength: Default::default(),
            connected: connected_masses,
        }
    }
}
