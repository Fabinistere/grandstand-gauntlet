use bevy::{prelude::*, utils::HashMap};
use bevy_rapier2d::prelude::*;

use crate::{
    camera::camera_follow,
    characters::movement::{MovementBundle, Speed},
    constants::character::{
        player::{
            BOTTOM_WHIP_POS_LEFT, BOTTOM_WHIP_POS_RIGHT, FRONT_WHIP_POS_LEFT, FRONT_WHIP_POS_RIGHT,
        },
        CHAR_POSITION,
    },
};

use super::{
    aggression::{AttackSensor, FlipAttackSensor},
    animations::{AnimationIndices, AnimationTimer, CharacterState},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app .add_startup_system(setup_player)
            // -- Camera --
            .add_system(camera_follow)
            // -- Aggression --
            .add_system(player_attack)
            // -- Movement --
            .add_system(player_movement)
            ;
    }
}

#[derive(Component)]
pub struct Player;

fn player_attack(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(Entity, &mut CharacterState), With<Player>>,
) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        // info!("DEBUG: return pressed");
        // eprintln!("DEBUG: BOM");
        let (_player, mut state) = player_query.single_mut();
        *state = CharacterState::Attack;
    }
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<
        (
            Entity,
            &Speed,
            &mut Velocity,
            &mut TextureAtlasSprite,
            &mut CharacterState,
        ),
        With<Player>,
    >,
    mut flip_direction_event: EventWriter<FlipAttackSensor>,
) {
    if let Ok((player, speed, mut rb_vel, mut texture_atlas_sprite, mut player_state)) =
        player_query.get_single_mut()
    {
        let left = keyboard_input.pressed(KeyCode::Q)
            || keyboard_input.pressed(KeyCode::Left)
            || keyboard_input.pressed(KeyCode::A);
        let right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);

        let x_axis = (right as i8) - left as i8;

        let mut vel_x = x_axis as f32 * **speed;

        if x_axis != 0 {
            vel_x *= (std::f32::consts::PI / 4.).cos();
        }

        rb_vel.linvel.x = vel_x;

        // ---- Animation ----

        // if there is any movement
        if (left || right) && *player_state != CharacterState::Run {
            *player_state = CharacterState::Run;
        } else if !(left || right) && *player_state == CharacterState::Run {
            *player_state = CharacterState::Idle;
        }

        // ---- Direction ----

        if (left && !texture_atlas_sprite.flip_x) || (right && texture_atlas_sprite.flip_x) {
            flip_direction_event.send(FlipAttackSensor(player));
        }

        // look where they are going - in the direction
        if right {
            // if texture_atlas_sprite.flip_x {
            //     flip_direction_event.send(FlipAttackSensor(player));
            // }
            texture_atlas_sprite.flip_x = false;
        } else if left {
            // if !texture_atlas_sprite.flip_x {
            //     flip_direction_event.send(FlipAttackSensor(player));
            // }
            texture_atlas_sprite.flip_x = true;
        }
    }
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut animation_indices = AnimationIndices(HashMap::new());
    animation_indices.insert(CharacterState::Idle, (0, 4));
    animation_indices.insert(CharacterState::Attack, (19, 23));
    animation_indices.insert(CharacterState::SecondAttack, (24, 26));
    animation_indices.insert(CharacterState::TransitionToCharge, (13, 14));
    animation_indices.insert(CharacterState::Charge, (15, 18));
    animation_indices.insert(CharacterState::Run, (5, 12));
    animation_indices.insert(CharacterState::Hit, (27, 28));
    animation_indices.insert(CharacterState::Dead, (29, 33));

    let texture_handle = asset_server.load("textures/character/character_spritesheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(122.0, 122.0), 34, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let texture_atlas_sprite = TextureAtlasSprite::new(0);

    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: texture_atlas_sprite,
                transform: Transform::from_translation(CHAR_POSITION.into()),
                ..default()
            },
            Player,
            Name::new("Player"),
            // -- Animation --
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            CharacterState::Idle,
            animation_indices,
            // -- Hitbox --
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            MovementBundle {
                speed: Speed::default(),
                velocity: Velocity {
                    linvel: Vect::ZERO,
                    angvel: 0.,
                },
            },
        ))
        .with_children(|parent| {
            // -- Attack Hitbox --
            parent
                .spawn((
                    SpatialBundle {
                        transform: Transform::from_translation(BOTTOM_WHIP_POS_RIGHT.into()),
                        ..default()
                    },
                    AttackSensor,
                    RigidBody::Dynamic,
                    Name::new("Bottom Whip Parent"),
                ))
                .with_children(|parent| {
                    // Thin bottom Whip
                    parent.spawn((
                        Collider::cuboid(21., 1.5),
                        Transform::default(),
                        Sensor,
                        ActiveEvents::COLLISION_EVENTS,
                        Name::new("Attack Hitbox: Sensor Bottom Whip"),
                    ));
                });

            parent
                .spawn((
                    SpatialBundle {
                        transform: Transform::from_translation(FRONT_WHIP_POS_RIGHT.into()),
                        ..default()
                    },
                    AttackSensor,
                    RigidBody::Dynamic,
                    Name::new("Parent Front Ball"),
                ))
                .with_children(|parent| {
                    // Front Ball
                    parent.spawn((
                        // Collider::segment(Vect::new(-50., 4.), Vect::new(-10., -10.)),
                        // Transform::default(),
                        Collider::cuboid(20., 7.),
                        Transform::default(),
                        Sensor,
                        ActiveEvents::COLLISION_EVENTS,
                        Name::new("Attack Hitbox: Sensor Front Ball"),
                    ));
                });

            // REFACTOR: Find a way to modify the transform of a sensor

            // ------- RIGHT ------
            // // Thin bottom Whip
            // parent.spawn((
            //     Collider::cuboid(21., 1.5),
            //     Transform::from_translation(BOTTOM_WHIP_POS_RIGHT.into()),
            //     // RigidBody::KinematicPositionBased,
            //     Sensor,
            //     AttackSensor,
            //     Name::new("Attack Hitbox: Thin bottom Whip"),
            // ));
            // // Front Ball
            // parent.spawn((
            //     Collider::cuboid(20., 7.),
            //     Transform::from_translation(FRONT_WHIP_POS_RIGHT.into()),
            //     // RigidBody::KinematicPositionBased,
            //     Sensor,
            //     AttackSensor,
            //     Name::new("Attack Hitbox: Front Ball"),
            // ));
        });
}
