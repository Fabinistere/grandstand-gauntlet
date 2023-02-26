pub mod aggression;
pub mod behaviors;
mod movement;

use bevy::{prelude::*, utils::HashMap};
use bevy_rapier2d::prelude::*;

use crate::{
    characters::{
        animations::{AnimationIndices, AnimationTimer, CharacterState},
        aggression::{Hp, AttackSensor, AttackHitbox, AttackCooldown},
        movement::{MovementBundle, Speed, CharacterHitbox},
        npcs::boss::{
            behaviors::*,
            aggression::{
                BossDeathEvent, boss_death, boss_attack_hitbox_activation, boss_proximity_attack, display_boss_hp
            },
            movement::{stare_player, chase_player}
        }
    },
    constants::character::{CHAR_POSITION, boss::{*, behaviors_sensors::*}, FRAME_TIME}, MySystems,
};

pub struct BossPlugin;

impl Plugin for BossPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app 
            .add_event::<BossDeathEvent>()
            .add_startup_system(setup_boss)
            // -- Movement --
            .add_system(chase_player)
            .add_system(stare_player)
            // -- UI --
            .add_system(display_boss_hp)
            // -- Aggression --
            .add_system(boss_proximity_attack)
            .add_system(
                boss_attack_hitbox_activation
                .label(MySystems::BossAttackHitboxActivation)
                .after(boss_proximity_attack)
            )
            .add_system(boss_death)
            // .add_plugin(AggressionBossPlugin) 
            // -- Behavior --
            .add_system(backstroke_sensor)
            ;
    }
}

#[derive(Component)]
pub struct Boss;

// -- Attack Hitbox --

#[derive(Component)]
pub struct BossAttack;

#[derive(Component)]
pub struct BossAttackSmash;

#[derive(Component)]
pub struct BossAttackFallenAngel;

fn setup_boss(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut animation_indices = AnimationIndices(HashMap::new());
    animation_indices.insert(CharacterState::Idle, BOSS_IDLE_FRAMES);
    animation_indices.insert(CharacterState::Run, BOSS_RUN_FRAMES);
    // Transition Blank to Smash (Backhand)
    animation_indices.insert(CharacterState::TransitionToCharge, BOSS_TRANSITION_TO_SMASH_FRAMES);
    // animation_indices.insert(CharacterState::Charge, BOSS_CHARGE_FRAMES);
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
            CharacterState::default(),
            animation_indices,
            AnimationTimer(Timer::from_seconds(FRAME_TIME, TimerMode::Repeating)),
            // -- Combat --
            Hp::new(BOSS_HP),
            AttackCooldown(Timer::from_seconds(
                BOSS_SMASH_COOLDOWN,
                TimerMode::Once,
            )),
            BossBehavior::Chase,
            // -- Movement --
            MovementBundle {
                speed: Speed::new(BOSS_SPEED),
                velocity: Velocity {
                    linvel: Vect::ZERO,
                    angvel: 0.,
                },
            },
            // -- Hitbox --
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
        ))
        .with_children(|parent| {
            // Boss Hitbox
            parent.spawn((
                Collider::ball(BOSS_HITBOX_SIZE),
                Transform::from_translation(BOSS_HITBOX_OFFSET_Y.into()),
                CharacterHitbox,
                Sensor,
                Name::new("Boss Hitbox"),
            ));

        // -- Sensors --

            // // Boss Movement Range Sensor: Chase Behavior
            // parent.spawn((
            //     Collider::ball(60.),
            //     Transform::from_translation(BOSS_HITBOX_OFFSET_Y.into()),
            //     Name::new("Boss Movement Range"),
            //     Sensor,
            //     BossSensor,
            //     BossMovementSensor,
            // ));

            // Boss Attack Range Sensor: Proximity sensor
            parent.spawn((
                // TODO: Bigger ?
                Collider::ball(BOSS_RANGE_HITBOX_SIZE),
                Transform::from_translation(BOSS_HITBOX_OFFSET_Y.into()),
                Name::new("Proximity Sensor: Smash"),
                // -- Standard --
                BossSensor,
                Sensor,
                // -- Custom --
                ProximitySensor,
            ));

            // Traitor moves Sensor: Backstroke Dash
            parent
                .spawn((
                    SpatialBundle {
                        transform: Transform::from_translation(BACKSTROKE_DASH_POS.into()),
                        ..default()
                    },
                    RigidBody::Dynamic,
                    Name::new("Parent - Backstroke Dash"),
                    BossSensor,
                ))
                .with_children(|parent| {
                    // This hierarchy is used to allow modification on the Sensor's transform
                    parent.spawn((
                        Collider::cuboid(
                            BACKSTROKE_DASH_SENSOR.0,
                            BACKSTROKE_DASH_SENSOR.1,
                        ),
                        Transform::default(),
                        Name::new("Backstroke Dash Sensor"),
                        Sensor,
                        BackstrokeDashSensor,
                    ));
                });
            

        // -- Attack Hitbox --
            // Smash
            parent
                .spawn((
                    SpatialBundle {
                        transform: Transform::from_translation(FRONT_SMASH_POS_TOP.into()),
                        ..default()
                    },
                    RigidBody::Dynamic,
                    Name::new("Parent - Smash Top"),
                    AttackSensor,
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
                    RigidBody::Dynamic,
                    Name::new("Parent - Smash Bot"),
                    AttackSensor,
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
                    RigidBody::Dynamic,
                    Name::new("Parent - FallenAngel"),
                    AttackSensor,
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
                        BossAttackFallenAngel,
                        // CollisionGroups::new(0b0100.into(), 0b0010.into()),
                        Sensor,
                        Name::new("Attack Hitbox: Sensor - FallenAnegel"),
                    ));
                });
        });
}
