use bevy::prelude::*;
use bevy_parallax::{LayerData, ParallaxMoveEvent, ParallaxResource};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum Location {
    Desert,
}

pub struct LocationsPlugin;

impl Plugin for LocationsPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app .add_state(Location::Desert)
            .insert_resource(ParallaxResource {
                layer_data: vec![
                    LayerData {
                        speed: 0.9,
                        path: "textures/map/Hills_Back.png".to_string(),
                        tile_size: Vec2::new(208., 256.),
                        cols: 1,
                        rows: 1,
                        scale: 0.55,
                        z: 0.0,
                        ..default()
                    },
                    LayerData {
                        speed: 0.8,
                        path: "textures/map/Hills Layer 02.png".to_string(),
                        tile_size: Vec2::new(512., 256.),
                        cols: 1,
                        rows: 1,
                        scale: 0.55,
                        z: 1.0,
                        ..default()
                    },
                    LayerData {
                        speed: 0.55,
                        path: "textures/map/Hills Layer 03.png".to_string(),
                        tile_size: Vec2::new(512., 256.),
                        cols: 1,
                        rows: 1,
                        scale: 0.55,
                        z: 2.0,
                        ..default()
                    },
                    LayerData {
                        speed: 0.5,
                        path: "textures/map/Hills Layer 04.png".to_string(),
                        tile_size: Vec2::new(512., 256.),
                        cols: 1,
                        rows: 1,
                        scale: 0.55,
                        z: 3.0,
                        ..default()
                    },
                    LayerData {
                        speed: 0.4,
                        path: "textures/map/Hills Layer 05.png".to_string(),
                        tile_size: Vec2::new(512., 256.),
                        cols: 1,
                        rows: 1,
                        scale: 0.55,
                        z: 4.0,
                        ..default()
                    },
                    LayerData {
                        speed: 0.2,
                        path: "textures/map/Hills Layer 06.png".to_string(),
                        tile_size: Vec2::new(512., 256.),
                        cols: 1,
                        rows: 1,
                        scale: 0.55,
                        z: 5.0,
                        ..default()
                    },
                ],
                ..default()
            })
            // .add_system_set(
            //     SystemSet::on_enter(Location::Desert)
            //         .with_system(setup_map)
            // )
            .add_system_set(
                SystemSet::on_update(Location::Desert)
                    // .with_run_criteria(run_if_in_level_one)
                    .with_system(move_camera_system)
            )
            ;
    }
}

// fn setup_map(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut texture_atlases: ResMut<Assets<TextureAtlas>>,
// ) {
// }

/// Send a ParallaxMoveEvent with the desired camera movement speed
pub fn move_camera_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut move_event_writer: EventWriter<ParallaxMoveEvent>,
) {
    if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
        move_event_writer.send(ParallaxMoveEvent {
            camera_move_speed: 3.,
        });
    } else if keyboard_input.pressed(KeyCode::Q)
        || keyboard_input.pressed(KeyCode::A)
        || keyboard_input.pressed(KeyCode::Left)
    {
        move_event_writer.send(ParallaxMoveEvent {
            camera_move_speed: -3.,
        });
    }
}
