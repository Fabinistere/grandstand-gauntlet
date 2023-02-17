mod aggression;
mod movement;

use bevy::{prelude::*, utils::HashMap};
use bevy_rapier2d::prelude::*;

use crate::{
    characters::{
        animations::{AnimationIndices, AnimationTimer, CharacterState},
        aggression::Hp,
        movement::{MovementBundle, Speed},
    },
    constants::character::{CHAR_POSITION, boss::*, FRAME_TIME},
};

use self::{
    aggression::{BossSensor, BossAttackEvent, boss_close_detection, boss_attack_event_handler, display_boss_hp},
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
            .add_system(display_boss_hp)
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
    animation_indices.insert(CharacterState::Idle, BOSS_IDLE_FRAMES);
    animation_indices.insert(CharacterState::Run, BOSS_RUN_FRAMES);
    // Charge to Backhand
    animation_indices.insert(CharacterState::TransitionToCharge, BOSS_TRANSITION_TO_CHARGE_FRAMES); //(11, 14)
    // Backhand
    animation_indices.insert(CharacterState::Charge, BOSS_CHARGE_FRAMES);
    // Powerfull Attack: Fallen angel
    animation_indices.insert(CharacterState::Attack, BOSS_FULL_ATTACK_FRAMES);
    // animation_indices.insert(CharacterState::SecondAttack, BOSS_SECOND_ATTACK_FRAMES);
    animation_indices.insert(CharacterState::Hit, BOSS_HIT_FRAMES);
    animation_indices.insert(CharacterState::Dead, BOSS_DEAD_FRAMES);

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
            AnimationTimer(Timer::from_seconds(FRAME_TIME, TimerMode::Repeating)),
            animation_indices,
            CharacterState::Idle,
            // -- Combat --
            Hp::new(BOSS_HP),
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
                Collider::ball(BOSS_HITBOX_SIZE),
                Transform::from_translation(BOSS_HITBOX_OFFSET_Y.into()),
                Sensor,
                ActiveEvents::COLLISION_EVENTS,
            ));

            // Boss Attack Sensor
            parent.spawn((
                Collider::ball(BOSS_RANGE_HITBOX_SIZE),
                Transform::from_translation(BOSS_HITBOX_OFFSET_Y.into()),
                Sensor,
                ActiveEvents::COLLISION_EVENTS,
                BossSensor,
            ));
        });
}
