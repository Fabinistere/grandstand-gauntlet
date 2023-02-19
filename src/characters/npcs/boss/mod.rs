mod aggression;
mod movement;

use bevy::{prelude::*, utils::HashMap};
use bevy_rapier2d::prelude::*;

use crate::{
    characters::{
        animations::{AnimationIndices, AnimationTimer, CharacterState},
        aggression::{Hp, AttackSensor, AttackHitbox, AttackCooldown},
        movement::{MovementBundle, Speed, CharacterHitbox},
    },
    constants::character::{CHAR_POSITION, boss::*, FRAME_TIME},
};

use self::{
    aggression::{BossSensor, BossAttackEvent, boss_attack_hitbox_activation, boss_close_detection, boss_attack_event_handler, display_boss_hp},
    movement::stare_player,
};

pub struct BossPlugin;

impl Plugin for BossPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app 
            .add_startup_system(setup_boss)
            .add_system(stare_player)
            .add_system(display_boss_hp)
            // -- Aggression --
            .add_event::<BossAttackEvent>()
            .add_system(boss_close_detection)
            .add_system(boss_attack_event_handler)
            .add_system(boss_attack_hitbox_activation.label("Boss Attack Hitbox Activation"))
            // .add_plugin(AggressionBossPlugin) 
            ;
    }
}

#[derive(Component)]
pub struct Boss;

#[derive(Component)]
pub struct BossAttack;

#[derive(Component)]
pub struct BossAttackSmash;

#[derive(Component)]
pub struct BossAttackFalleAngel;

fn setup_boss(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut animation_indices = AnimationIndices(HashMap::new());
    animation_indices.insert(CharacterState::Idle, BOSS_IDLE_FRAMES);
    animation_indices.insert(CharacterState::Run, BOSS_RUN_FRAMES);
    // animation_indices.insert(CharacterState::TransitionToCharge, BOSS_TRANSITION_TO_CHARGE_FRAMES); //(11, 14)
    // Charge to Backhand
    animation_indices.insert(CharacterState::Charge, BOSS_CHARGE_FRAMES);
    // Backhand
    animation_indices.insert(CharacterState::Attack, BOSS_SMASH_FRAMES);
    // Powerfull Attack: Fallen angel
    animation_indices.insert(CharacterState::SecondAttack, BOSS_FALLEN_ANGEL_FRAMES);
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
            CharacterState::default(),
            // -- Combat --
            Hp::new(BOSS_HP),
            AttackCooldown(Timer::from_seconds(
                BOSS_SMASH_COOLDOWN,
                TimerMode::Once,
            )),
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
                CharacterHitbox,
                Sensor,
                // ActiveEvents::COLLISION_EVENTS,
                Name::new("Boss Hitbox"),
            ));

            // Boss Attack Range Sensor
            parent.spawn((
                Collider::ball(BOSS_RANGE_HITBOX_SIZE),
                Transform::from_translation(BOSS_HITBOX_OFFSET_Y.into()),
                BossSensor,
                Sensor,
                ActiveEvents::COLLISION_EVENTS,
                Name::new("Boss Attack Range"),
            ));

            // -- Attack Hitbox --
            // Smash
            parent
                .spawn((
                    SpatialBundle {
                        transform: Transform::from_translation(FRONT_SMASH_POS_TOP.into()),
                        ..default()
                    },
                    AttackSensor,
                    RigidBody::Dynamic,
                    Name::new("Parent - Smash Top"),
                ))
                .with_children(|parent| {
                    // Front
                    parent.spawn((
                        // REFACTOR: Find a way to .into() a (f32, f32) tuple into a 2arguments function
                        Collider::cuboid(
                            BOSS_ATTACK_HITBOX_SMASH_TOP.0,
                            BOSS_ATTACK_HITBOX_SMASH_TOP.1,
                        ),
                        Transform::default(),
                        AttackHitbox(10),
                        BossAttack,
                        BossAttackSmash,
                        Sensor,
                        Name::new("Attack Hitbox: Sensor - Smash Top"),
                    ));
                });
            parent
                .spawn((
                    SpatialBundle {
                        transform: Transform::from_translation(FRONT_SMASH_POS_BOTTOM.into()),
                        ..default()
                    },
                    AttackSensor,
                    RigidBody::Dynamic,
                    Name::new("Parent - Smash Bot"),
                ))
                .with_children(|parent| {
                    // Front
                    parent.spawn((
                        // REFACTOR: Find a way to .into() a (f32, f32) tuple into a 2arguments function
                        Collider::cuboid(
                            BOSS_ATTACK_HITBOX_SMASH_BOTTOM.0,
                            BOSS_ATTACK_HITBOX_SMASH_BOTTOM.1,
                        ),
                        Transform::default(),
                        AttackHitbox(10),
                        BossAttack,
                        BossAttackSmash,
                        Sensor,
                        Name::new("Attack Hitbox: Sensor - Smash Bot"),
                    ));
                });

            // Fallen Angel
            parent
                .spawn((
                    SpatialBundle {
                        transform: Transform::from_translation(FALLEN_ANGEL_POS.into()),
                        ..default()
                    },
                    AttackSensor,
                    RigidBody::Dynamic,
                    Name::new("Parent - FallenAngel"),
                ))
                .with_children(|parent| {
                    // Front
                    parent.spawn((
                        // REFACTOR: Find a way to .into() a (f32, f32) tuple into a 2arguments function
                        Collider::cuboid(
                            BOSS_ATTACK_HITBOX_FALLEN_ANGEL.0,
                            BOSS_ATTACK_HITBOX_FALLEN_ANGEL.1,
                        ),
                        Transform::default(),
                        AttackHitbox(10),
                        BossAttack,
                        BossAttackFalleAngel,
                        // CollisionGroups::new(0b0100.into(), 0b0010.into()),
                        Sensor,
                        Name::new("Attack Hitbox: Sensor - FallenAnegel"),
                    ));
                });
        });
}
