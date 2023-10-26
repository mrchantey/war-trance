use std::ops::Deref;
use std::ops::DerefMut;

use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Component)]
pub struct Velocity {
    pub value: Vec3,
    pub max: f32,
}

impl Default for Velocity {
    fn default() -> Self {
        Self {
            value: Vec3::ZERO,
            max: 1.0,
        }
    }
}

impl Deref for Velocity {
    type Target = Vec3;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for Velocity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

pub fn velocity_system(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        if velocity.value != Vec3::ZERO {
            transform.translation += velocity.value * time.delta_seconds();
        }
    }
}
