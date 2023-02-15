#![allow(clippy::type_complexity)]

pub mod camera;
pub mod constants;
mod crowd;
mod debug;
mod locations;
pub mod movement;
mod player;

use bevy::prelude::*;
use bevy_parallax::{ParallaxCameraComponent, ParallaxPlugin};
use bevy_rapier2d::prelude::*;

use constants::{CLEAR, TILE_SIZE};
use debug::DebugPlugin;
use locations::LocationsPlugin;
use player::PlayerPlugin;

#[rustfmt::skip]
fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(Msaa { samples: 1 })
        // v-- Hitbox --v
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Grandstand Gauntlet".to_string(),
                        ..default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(RapierDebugRenderPlugin {
            mode: DebugRenderMode::all(),
            ..default()
        })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            TILE_SIZE,
        ))
        .add_plugin(ParallaxPlugin)
        .add_plugin(LocationsPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(crowd::CrowdPlugin)
        .add_startup_system(spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.2;

    commands.spawn(camera).insert(ParallaxCameraComponent);
}
