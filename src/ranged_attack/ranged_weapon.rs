use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct RangedWeapon {
    pub range: f32,
    pub windup: Duration,
}

impl Default for RangedWeapon {
    fn default() -> Self {
        Self {
            range: 3.,
            windup: Duration::from_secs(1),
        }
    }
}
