mod player;

use bevy::prelude::*;

use bevy_parallax::{ParallaxCameraComponent, ParallaxPlugin};
use constants::CLEAR;
use locations::LocationsPlugin;

pub mod constants;
mod locations;

#[rustfmt::skip]
fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(Msaa { samples: 1 })
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
        .add_plugin(ParallaxPlugin)
        .add_plugin(LocationsPlugin)
        .add_startup_system(spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scale = 0.2;

    commands.spawn(camera).insert(ParallaxCameraComponent);
}
