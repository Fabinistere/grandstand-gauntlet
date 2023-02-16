//! A big gouchi Boss

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::characters::{npcs::boss::Boss, player::Player, Invulnerable};

/// DOC
/// 
/// Happens when:
///   - character::npcs:::boss_attack
///     - actions
/// Read in
///   - character::npcs::aggression::???
///     - actions
pub struct BossAttackEvent {
    npc_entity: Entity,
    target_entity: Entity,
}

#[derive(Component)]
pub struct BossSensor;

/// When the player enters the sensor
/// The boss start to attack them
///
/// Send a Event to launch a attack when a entity enters the sensor
/// and insert a timer to limit the number of attack while still in the sensor.
///
/// Remove this timer (and behavior?) when leaving the sensor
///
/// ***IMPORTANT***:
/// - when dying/tp the exit trigger of the sensor will not trigger
/// Has to verify that the entity is nearby
/// OR
/// when tp/dying->soul shift the player become Invulnerable
/// - If no player vulnerable, then remove the behavior
pub fn boss_attack(
    mut collision_events: EventReader<CollisionEvent>,
    rapier_context: Res<RapierContext>,

    boss_attack_sensor: Query<(Entity, &Parent), (With<Sensor>, With<BossSensor>)>,

    mut boss_query: Query<(&mut TextureAtlasSprite, &Transform), With<Boss>>,
    mut player_query: Query<(Entity, &Transform), (With<Player>, Without<Invulnerable>)>,
) {
    // TODO: Phase 1 - Sensor
    for collision_event in collision_events.iter() {
        let entity_1 = collision_event.entities().0;
        let entity_2 = collision_event.entities().1;

        if rapier_context.intersection_pair(entity_1, entity_2) == Some(true) {
            match (boss_attack_sensor.get(e1), boss_attack_sensor.get(e2)) {
                // (Err(e1), Err(e2)) => continue,
                (Ok((attack_sensor, boss)), Err(_)) | (Err(_), Ok((attack_sensor, boss))) => {

                }
                _ => continue,

            }
        }
    }
    // TODO: Phase 2 - Timer
    // TODO: Phase 3 - TP proof
}
