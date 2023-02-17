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

use self::{
    aggression::{BossSensor, BossAttackEvent, boss_close_detection, boss_attack_event_handler},
    movement::stare_player,
};

pub struct BossPlugin;

impl Plugin for BossPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app 
            .add_startup_system(setup_boss)
            .add_system(stare_player)
            // -- Aggression --
            .add_event::<BossAttackEvent>()
            .add_system(boss_close_detection)
            .add_system(boss_attack_event_handler)
            // .add_plugin(AggressionBossPlugin) 
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
    animation_indices.insert(CharacterState::Run, (5, 10));
    // Charge to Backhand
    animation_indices.insert(CharacterState::TransitionToCharge, (11, 18)); //(11, 14)
    // Backhand
    animation_indices.insert(CharacterState::Charge, (15, 18));
    // Powerfull Attack: Fallen angel
    animation_indices.insert(CharacterState::Attack, (19, 26));
    // animation_indices.insert(CharacterState::SecondAttack, (24, 26));
    animation_indices.insert(CharacterState::Hit, (27, 28));
    animation_indices.insert(CharacterState::Dead, (29, 34));

    let texture_handle = asset_server.load("textures/character/magic_bot_spritesheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(200.0, 200.0), 35, 1, None, None);
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
        ))
        .with_children(|parent| {
            // Boss Hitbox
            parent.spawn((
                Collider::ball(12.),
                // OFFSET_Y
                Transform::from_translation((0., 5., 0.).into()),
                Sensor,
                ActiveEvents::COLLISION_EVENTS,
            ));

            // Boss Attack Sensor
            parent.spawn((
                Collider::ball(40.),
                // OFFSET_Y
                Transform::from_translation((0., 5., 0.).into()),
                Sensor,
                ActiveEvents::COLLISION_EVENTS,
                BossSensor,
            ));
        });
}
