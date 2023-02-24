use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::Velocity;
// use bevy_retrograde::prelude::Velocity;

use crate::TILE_SIZE;

// find the right place to put this component (indicator)
#[derive(Component)]
pub struct CharacterHitbox;

#[derive(Component, Deref, DerefMut, Inspectable)]
pub struct Speed(pub f32);

impl Default for Speed {
    fn default() -> Self {
        Speed(50. * TILE_SIZE)
    }
}

impl Speed {
    pub fn new(speed: f32) -> Self {
        Speed(speed * TILE_SIZE)
    }
}

#[derive(Bundle)]
pub struct MovementBundle {
    pub speed: Speed,
    pub velocity: Velocity,
}
