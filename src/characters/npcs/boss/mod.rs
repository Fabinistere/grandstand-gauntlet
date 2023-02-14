mod aggression;
mod movement;

use bevy::{prelude::*, utils::HashMap};
use bevy_rapier2d::prelude::*;

use crate::{
    characters::{
        animations::{AnimationIndices, AnimationTimer, CharacterState},
        movement::{MovementBundle, Speed},
    },
    constants::character::CHAR_POSITION,
};

use self::movement::stare_player;

pub struct BossPlugin;

impl Plugin for BossPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app .add_startup_system(setup_boss)
            .add_system(stare_player)
            ;
    }
}

#[derive(Component)]
pub struct Boss;

fn setup_boss(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut animation_indices = AnimationIndices(HashMap::new());
    animation_indices.insert(CharacterState::Idle, (0, 4));
    animation_indices.insert(CharacterState::Attack, (19, 26));
    animation_indices.insert(CharacterState::SecondAttack, (24, 26));
    animation_indices.insert(CharacterState::TransitionToCharge, (11, 14));
    animation_indices.insert(CharacterState::Charge, (15, 18));
    animation_indices.insert(CharacterState::Run, (5, 10));
    animation_indices.insert(CharacterState::Hit, (27, 28));
    animation_indices.insert(CharacterState::Dead, (29, 34));

    let texture_handle = asset_server.load("textures/character/magic_bot_spritesheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(200.0, 200.0), 35, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let texture_atlas_sprite = TextureAtlasSprite::new(0);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: texture_atlas_sprite,
            transform: Transform::from_translation(CHAR_POSITION.into()),
            ..default()
        },
        Boss,
        Name::new("Boss"),
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