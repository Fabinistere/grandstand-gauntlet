#![allow(clippy::type_complexity)]

pub mod camera;
pub mod characters;
pub mod collisions;
pub mod constants;
mod crowd;
mod debug;
mod locations;
mod soul_shift;
mod ui;

use bevy::prelude::*;
use bevy_parallax::{ParallaxCameraComponent, ParallaxPlugin};
use bevy_rapier2d::prelude::*;
// use std::env;

use constants::{CLEAR, TILE_SIZE};

/// See Usage and example here: [System Label](https://bevy-cheatbook.github.io/programming/labels.html)
///
/// # Note
///
/// DOC: Name it better
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum MySystems {
    /// The Player launched a attack
    PlayerAttackHitboxActivation,
    /// The Boss launched a attack
    BossAttackHitboxActivation,
    /// A AttackHitbox touches a CharacterHitbox
    AttackCollision,
    DamageHit,
    DamageAnimation,
    /// Soul Shift Event Handler
    ///
    /// End the dying current player (TODO: it shouldn't) and
    /// Respawn them into the closest crowd member
    SoulShift,
    /// Give all require component to the new player
    NewBeginning,
    /// End the dying past player
    PlayerDeath,
}

#[rustfmt::skip]
fn main() {
    // env::set_var("RUST_BACKTRACE", "FULL");

    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(Msaa { samples: 1 })
        .add_plugins(
            DefaultPlugins
            .set(WindowPlugin {
                window: WindowDescriptor {
                    fit_canvas_to_parent: true,
                    title: "Grandstand Gauntlet".to_string(),
                    ..default()
                },
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
        )
        // v-- Hitbox --v
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .add_plugin(RapierDebugRenderPlugin {
            mode: DebugRenderMode::all(),
            ..default()
        })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            TILE_SIZE,
        ))
        // v-- The GAME --v
        .add_plugin(ParallaxPlugin)
        .add_plugin(characters::CharacterPlugin)
        .add_plugin(crowd::CrowdPlugin)
        .add_plugin(debug::DebugPlugin)
        .add_plugin(locations::LocationsPlugin)
        .add_plugin(soul_shift::SoulShiftPlugin)
        .add_plugin(ui::UiPlugin)
        .add_startup_system(spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.2;

    commands.spawn(camera).insert(ParallaxCameraComponent);
}
