use bevy::{prelude::*, utils::HashMap};
use bevy_rapier2d::prelude::*;

use crate::{
    constants::character::CHAR_POSITION,
    movement::{MovementBundle, Speed},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app .add_startup_system(setup_player)
            // -- Animation --
            .add_system(animate_player)
            .add_system(jump_frame_player_state)
            // -- Aggression --
            .add_system(player_attack)
            // -- Movement --
            .add_system(player_movement)
            ;
    }
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct AnimationIndices(HashMap<PlayerState, (usize, usize)>);

#[derive(Component)]
pub struct Player;

#[derive(Component, PartialEq, Eq, Hash)]
enum PlayerState {
    Idle,
    Attack,
    SecondAttack,
    TransitionToCharge,
    Charge,
    Run,
    Hit,
    Dead,
}

fn player_attack(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(Entity, &mut PlayerState), With<Player>>,
) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        info!("return pressed");
        let (_player, mut state) = player_query.single_mut();
        *state = PlayerState::Attack;
    }
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<
        (
            &Speed,
            &mut Velocity,
            &mut TextureAtlasSprite,
            &mut PlayerState,
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
        if (left || right) && *player_state != PlayerState::Run {
            *player_state = PlayerState::Run;
        } else if !(left || right) && *player_state == PlayerState::Run {
            *player_state = PlayerState::Idle;
        }

        // look where they are going - in the direction
        if right {
            texture_atlas_sprite.flip_x = false;
        } else if left {
            texture_atlas_sprite.flip_x = true;
        }
    }
}

fn animate_player(
    time: Res<Time>,
    mut query: Query<
        (
            &AnimationIndices,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &mut PlayerState,
        ),
        With<Player>,
    >,
) {
    for (indices, mut timer, mut sprite, mut player_state) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            let indices = indices[&player_state];
            // REFACTOR: the limit being modified by magical number
            let limit: usize;
            let start_back: usize;
            let state_when_restart: Option<PlayerState>;

            if *player_state == PlayerState::Attack || *player_state == PlayerState::SecondAttack {
                // Idle
                start_back = 0;
                state_when_restart = Some(PlayerState::Idle);
                // End of SecondAttack
                limit = 26;
            } else if *player_state == PlayerState::Run {
                // Idle
                start_back = 0;
                state_when_restart = Some(PlayerState::Idle);
                // End of SecondAttack
                limit = 12;
            } else {
                // Loop
                start_back = indices.0;
                state_when_restart = None;
                limit = indices.1;
            }

            if sprite.index == limit {
                sprite.index = start_back;
                // update state
                match state_when_restart {
                    Some(new_state) => *player_state = new_state,
                    None => continue,
                }
            } else {
                sprite.index = sprite.index + 1
            };
        }
    }
}

/// Anytime the PlayerState change,
/// force the sprite to match this change.
fn jump_frame_player_state(
    mut query: Query<
        (&AnimationIndices, &mut TextureAtlasSprite, &PlayerState),
        (With<Player>, Changed<PlayerState>),
    >,
) {
    for (indices, mut sprite, player_state) in &mut query {
        let indices = indices[&player_state];
        // Jump directly to the correct frame
        sprite.index = indices.0;
    }
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut animation_indices = AnimationIndices(HashMap::new());
    animation_indices.insert(PlayerState::Idle, (0, 4));
    animation_indices.insert(PlayerState::Attack, (19, 23));
    animation_indices.insert(PlayerState::SecondAttack, (24, 26));
    animation_indices.insert(PlayerState::TransitionToCharge, (13, 14));
    animation_indices.insert(PlayerState::Charge, (15, 18));
    animation_indices.insert(PlayerState::Run, (5, 12));
    animation_indices.insert(PlayerState::Hit, (27, 28));
    animation_indices.insert(PlayerState::Dead, (29, 33));

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
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        animation_indices,
        PlayerState::Idle,
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
