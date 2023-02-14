use bevy::prelude::*;

use crate::characters::{npcs::boss::Boss, player::Player, Invulnerable};

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
    mut boss_query: Query<(&mut TextureAtlasSprite, &Transform), With<Boss>>,
    mut player_query: Query<(Entity, &Transform), (With<Player>, Without<Invulnerable>)>,
) {
    // TODO: Phase 1 - Sensor
    // TODO: Phase 2 - Timer
    // TODO: Phase 3 - TP proof
}
