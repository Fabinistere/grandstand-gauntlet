use bevy::{prelude::*, utils::HashMap};
use bevy_rapier2d::prelude::*;

use crate::{
    camera::camera_follow,
    characters::movement::{MovementBundle, Speed},
    constants::character::CHAR_POSITION,
};

use super::animations::{AnimationIndices, AnimationTimer, CharacterState};

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
        let (_player, mut state) = player_query.single_mut();
        *state = CharacterState::Attack;
    }
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<
        (
            &Speed,
            &mut Velocity,
            &mut TextureAtlasSprite,
            &mut CharacterState,
        ),
        With<Player>,
    >,
) {
    if let Ok((speed, mut rb_vel, mut texture_atlas_sprite, mut player_state)) =
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

        // if there is any movement
        if (left || right) && *player_state != CharacterState::Run {
            *player_state = CharacterState::Run;
        } else if !(left || right) && *player_state == CharacterState::Run {
            *player_state = CharacterState::Idle;
        }

        // look where they are going - in the direction
        if right {
            texture_atlas_sprite.flip_x = false;
        } else if left {
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

    commands.spawn((
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
        animation_indices,
        CharacterState::Idle,
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
    ));
}
