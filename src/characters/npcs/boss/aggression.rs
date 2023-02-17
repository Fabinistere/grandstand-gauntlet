//! A big gouchi Boss

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    characters::{
        animations::CharacterState,
        // npcs::boss::Boss,
        player::PlayerHitbox,
        // Invulnerable,
    },
    collisions::CollisionEventExt,
};

pub struct AggressionBossPlugin;

impl Plugin for AggressionBossPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app .add_event::<BossAttackEvent>()
            .add_system(boss_close_detection)
            .add_system(boss_attack_event_handler)
            ;
    }
}

/// DOC
///
/// Happens when:
///   - character::npcs::boss::boss_close_detection
///     - The player hitbox/Sensor enters the BossSensor
/// Read in
///   - character::npcs::boss::aggression::boss_attack_event_handler
///     - Launch an attack to the current facing direction
///     (cause they always look at the player)
pub struct BossAttackEvent {
    attacker_entity: Entity,
    // target_entity: Entity,
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
///
/// # Note
///
/// The boss should still attack while the player is invulnerable;
///
/// And we could just deactivate the player hitbox if Invulnerable by removing `ActiveEvents`
///
/// ^^^----- I don't know if that could just work: (if not) for more depts: [Collision groups and solver groups](https://rapier.rs/docs/user_guides/bevy_plugin/colliders/#collision-groups-and-solver-groups)
pub fn boss_close_detection(
    mut collision_events: EventReader<CollisionEvent>,
    rapier_context: Res<RapierContext>,

    boss_attack_sensor_query: Query<(Entity, &Parent), (With<Sensor>, With<BossSensor>)>,
    player_sensor_query: Query<(Entity, &Parent), With<PlayerHitbox>>,

    // mut boss_query: Query<Entity, With<Boss>>,
    mut boss_attack_event: EventWriter<BossAttackEvent>,
) {
    // TODO: Phase 1 - Sensor
    for collision_event in collision_events.iter() {
        let entity_1 = collision_event.entities().0;
        let entity_2 = collision_event.entities().1;

        // info!("DEBUG: {:?} x {:?}", entity_1, entity_2);

        if rapier_context.intersection_pair(entity_1, entity_2) == Some(true) {
            match (
                boss_attack_sensor_query.get(entity_1),
                boss_attack_sensor_query.get(entity_2),
                player_sensor_query.get(entity_1),
                player_sensor_query.get(entity_2),
            ) {
                // (Err(eboss1), Err(eboss2), Err(eplayer1), Err(eplayer2)) => continue,
                (Ok((_attack_sensor, boss)), Err(_), Err(_), Ok((_player_sensor, _player)))
                | (Err(_), Ok((_attack_sensor, boss)), Ok((_player_sensor, _player)), Err(_)) => {
                    // IDEA: MUST-HAVE - Disable turn/movement when the boss attack (avoid spinning attack when passing behind the boss)

                    // info!("DEBUG: Detected");

                    // let boss_entity = boss_query.single_mut();
                    // if boss_entity != **boss { // _potential
                    //     warn!("BossSensor on something else than a Boss");
                    //     break;
                    // }
                    boss_attack_event.send(BossAttackEvent {
                        attacker_entity: **boss, // boss_entity
                                                 // target_entity: **player,
                    });
                }
                _ => continue,
            }
        }
        // Leaving the range
        else {
            // IDEA: reset the BAM_timer
        }
    }
    // TODO: Phase 2 - Timer
    // TODO: Phase 3 - Player TP proof
}

fn boss_attack_event_handler(
    mut boss_attack_event: EventReader<BossAttackEvent>,
    // If needed to check the Player Invulnerability state:
    // Without<Invulnerable>
    // player_query: Query<Entity, With<Player>>,

    // &mut TextureAtlasSprite With<Boss>
    mut attacker_query: Query<&mut CharacterState>,
) {
    for BossAttackEvent {
        attacker_entity,
        // target_entity,
    } in boss_attack_event.iter()
    {
        match attacker_query.get_mut(*attacker_entity) {
            // DEBUG: (in the start of the game) / Every time a entity spawns, log the name + current identifier
            Err(e) => warn!(
                "This entity: {:?} Cannot animate: {:?}",
                *attacker_entity, e
            ),
            Ok(mut state) => {
                *state = CharacterState::Attack;
            }
        }
    }
}
