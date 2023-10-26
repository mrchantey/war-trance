use std::ops::Deref;
use std::ops::DerefMut;

use bevy::prelude::*;

#[derive(Debug, Copy, Clone, Component)]
pub struct Health {
    pub value: f32,
    pub max: f32,
}

impl DerefMut for Health {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl Deref for Health {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Default for Health {
    fn default() -> Self {
        Self::new(100., 100.)
    }
}

impl Health {
    pub fn new(value: f32, max: f32) -> Self {
        Self { value, max }
    }
}
