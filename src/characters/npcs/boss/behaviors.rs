//! All methods only linked to the detection Sensor/trigger Boss Behavior
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;

use crate::{
    characters::{
        movement::CharacterHitbox,
        player::{Player, PlayerHitbox},
    },
    collisions::CollisionEventExt,
};

use super::Boss;

// -- Boss Behaviors --

#[derive(Default, Debug, Clone, Component, Eq, Inspectable, PartialEq)]
pub enum BossBehavior {
    #[default]
    Chase,
    /// # Trigger
    ///
    /// The player runs away, after leaving the proximity sensor
    /// (negative movement velocity % Boss)
    DashInAttack,
    /// Tp in front/behind alternately, each tp is followed by a fake Smash (just the transition)
    ///
    /// # Trigger
    ///
    /// x successed paries
    CounterPary,
    /// Dash out, Dash in, Smash Attack
    ///
    /// # Trigger
    ///
    /// Player being too close to the boss
    NeedSomeSpace,
}

// -- Behaviors Sensor --

/// Groups all Boss' Sensors
#[derive(Component)]
pub struct BossSensor;

// /// Range which indicates that the boss is nearby the player
// #[derive(Component)]
// pub struct BossMovementSensor;

/// If the player is in this sensor, the boss attacks only if they aren't in cooldown
#[derive(Component)]
pub struct ProximitySensor;

/// If the player is in this sensor, the boss attacks only if they aren't in cooldown
#[derive(Component)]
pub struct BackstrokeDashSensor;

// ------------------------------------------------------------

/// Detects if a player is running away from the boss
///
/// It will then trigger a `Dash In` into `Smash Attack`
///
/// # Note
///
/// ## Be Normal
///
/// Full Dash Anim + FX in the spritesheet, centered,
/// And just applya big Velocity will match the fx
///
/// ## If we care about the perfect dash tray in case of a dashwall
///
/// For each `x` transform traveled,
/// spawn on it particles which will decay (spritesheet).
/// Two type, the first
pub fn backstroke_sensor(
    mut collision_events: EventReader<CollisionEvent>,

    rapier_context: Res<RapierContext>,

    backstroke_sensor_query: Query<Entity, With<BackstrokeDashSensor>>,
    player_hitbox_query: Query<Entity, (With<PlayerHitbox>, With<CharacterHitbox>)>,

    player_query: Query<(&Transform, &Velocity), With<Player>>,
    mut boss_query: Query<(&mut BossBehavior, &Transform), With<Boss>>,
) {
    for collision_event in collision_events.iter() {
        let entity_1 = collision_event.entities().0;
        let entity_2 = collision_event.entities().1;

        if rapier_context.intersection_pair(entity_1, entity_2) == Some(true) {
            match (
                backstroke_sensor_query.get(entity_1),
                backstroke_sensor_query.get(entity_2),
                player_hitbox_query.get(entity_1),
                player_hitbox_query.get(entity_2),
            ) {
                (Ok(_), Err(_), Err(_), Ok(_)) | (Err(_), Ok(_), Ok(_), Err(_)) => {
                    let (player_transform, player_vel) = player_query.single();
                    let (mut boss_behavior, boss_transform) = boss_query.single_mut();

                    // The player is running away
                    if (boss_transform.translation.x < player_transform.translation.x
                        && player_vel.linvel.x < 0.)
                        || (boss_transform.translation.x > player_transform.translation.x
                            && player_vel.linvel.x > 0.)
                    {
                        *boss_behavior = BossBehavior::DashInAttack;
                    }
                }
                _ => continue,
            }
        }
    }
}
