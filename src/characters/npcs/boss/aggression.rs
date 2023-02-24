//! A big gouchi Boss

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    characters::{
        aggression::{AttackCooldown, AttackHitbox, AttackSensor, Hp},
        // Invulnerable,
        animations::CharacterState,
        movement::CharacterHitbox,
        player::{PlayerHitbox, PossesionCount},
    },
    // collisions::CollisionEventExt,
    constants::character::boss::BOSS_SMASH_COOLDOWN,
};

use super::{Boss, BossAttackFalleAngel, BossAttackSmash};

// pub struct AggressionBossPlugin;

// impl Plugin for AggressionBossPlugin {
//     #[rustfmt::skip]
//     fn build(&self, app: &mut App) {
//         app .add_event::<BossAttackEvent>()
//             .add_system(boss_close_detection)
//             .add_system(boss_attack_event_handler)
//             ;
//     }
// }

// DOC: move it up with and named it better
#[derive(Component)]
pub struct BossSensor;

/// Happens when
///   - ???
///     - action
///
/// Read in
///   - ???
///     - action
pub struct BossDeathEvent;

/// DEBUG: TEMPORARY
///
/// The Boss' hp won't be displayed.
/// The current phase will indicate, as well as the clouds ?
pub fn display_boss_hp(
    bleeding_boss_query: Query<&Hp, (With<Boss>, Or<(Added<Hp>, Changed<Hp>)>)>,
) {
    if let Ok(boss_hp) = bleeding_boss_query.get_single() {
        println!("boss's hp: {}/{}", boss_hp.current, boss_hp.max);
    }
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
///
/// # Note
///
/// The boss should still attack while the player is invulnerable;
/// For more depts: [Collision groups and solver groups](https://rapier.rs/docs/user_guides/bevy_plugin/colliders/#collision-groups-and-solver-groups)
pub fn boss_close_detection(
    mut commands: Commands,

    // mut collision_events: EventReader<CollisionEvent>,
    rapier_context: Res<RapierContext>,

    boss_attack_sensor_query: Query<
        (Entity, &Parent),
        (With<Sensor>, With<BossSensor>, Without<AttackCooldown>),
    >,
    player_sensor_query: Query<(Entity, &Parent), (With<PlayerHitbox>, With<CharacterHitbox>)>,

    mut attacker_query: Query<&mut CharacterState>,
) {
    // Phase 1 - Sensor
    if let Ok((attack_sensor, boss)) = boss_attack_sensor_query.get_single() {
        if let Ok((player_sensor, _player)) = player_sensor_query.get_single() {
            // Phase 3 - Player TP proof
            if rapier_context.intersection_pair(attack_sensor, player_sensor) == Some(true) {
                match attacker_query.get_mut(**boss) {
                    // DEBUG: (in the start of the game) / Every time a entity spawns, log the name + current identifier
                    Err(e) => warn!("This entity: {:?} Cannot be animated: {:?}", **boss, e),
                    Ok(mut state) => {
                        *state = CharacterState::TransitionToCharge;
                    }
                }

                // Phase 2 - Timer
                commands
                    // REFACTOR: Where the cooldown timer is placed
                    .entity(attack_sensor) // **boss
                    .insert(AttackCooldown(Timer::from_seconds(
                        BOSS_SMASH_COOLDOWN,
                        TimerMode::Once,
                    )));
            }
        }
        // else { info!("No playerHitbox") }
    }
}

/// Activate when the character is on animation phase Attack,
/// Deactivate else.
pub fn boss_attack_hitbox_activation(
    mut commands: Commands,

    boss_query: Query<(&CharacterState, &Children, &Name), (Changed<CharacterState>, With<Boss>)>,
    parent_hitbox_position_query: Query<(Entity, &Children), With<AttackSensor>>,

    // All Kind of Boss Attack
    smash_hitbox_query: Query<Entity, (With<AttackHitbox>, With<BossAttackSmash>, With<Sensor>)>,
    fallen_angel_hitbox_query: Query<
        Entity,
        (With<AttackHitbox>, With<BossAttackFalleAngel>, With<Sensor>),
    >,
) {
    for (character_state, children, _name) in boss_query.iter() {
        // info!("DEBUG: {} Changed {:?}", name, *character_state);
        for child in children.iter() {
            match parent_hitbox_position_query.get(*child) {
                Err(_) => continue,
                // The parent Hitbox contains the modifiable transform
                // for all their hitbox children
                Ok((_parent_hitbox, hitbox_children)) => {
                    for hitbox_child in hitbox_children.iter() {
                        // OPTIMIZE: Hitbox Activation - This statement will be called a bit too much
                        // vv-- to just see uncomment the two DEBUG info below --vv

                        // Just activate the good group of AttackHitbox (Smash or FallenAngel)
                        if *character_state == CharacterState::Attack
                            || *character_state == CharacterState::SecondAttack
                        {
                            match (
                                smash_hitbox_query.get(*hitbox_child),
                                fallen_angel_hitbox_query.get(*hitbox_child),
                            ) {
                                (Ok(smash), Err(_)) => {
                                    if *character_state == CharacterState::Attack {
                                        // info!("DEBUG: Smash Active Inserted on {}", _name);
                                        commands
                                            .entity(smash)
                                            .insert(ActiveEvents::COLLISION_EVENTS);
                                    } else {
                                        // info!("DEBUG: Smash Active Removed on {}", _name);
                                        commands.entity(smash).remove::<ActiveEvents>();
                                    }
                                }
                                (Err(_), Ok(fallen_angel)) => {
                                    if *character_state == CharacterState::SecondAttack {
                                        // info!("DEBUG: Fallen Angel Active Inserted on {}", _name);
                                        commands
                                            .entity(fallen_angel)
                                            .insert(ActiveEvents::COLLISION_EVENTS);
                                    } else {
                                        // info!("DEBUG: Fallen Angel Active Removed on {}", _name);
                                        commands.entity(fallen_angel).remove::<ActiveEvents>();
                                    }
                                }
                                (Err(_), Err(_)) => {
                                    warn!("Non Indexed Attack (Smash or Fallen Angel)")
                                }
                                (Ok(_), Ok(_)) => {
                                    warn!("Attack indexed twice (Smash and Fallen Angel)")
                                }
                            }
                        } else {
                            // info!("DEBUG: Any Active Removed on {}", _name);
                            commands.entity(*hitbox_child).remove::<ActiveEvents>();
                        }

                        // match attack_hitbox_query.get(*child)
                    }
                }
            }
        }
    }
}

/// # Note
///
/// TODO: End of the Game
pub fn boss_death(
    mut boss_death_event: EventReader<BossDeathEvent>,
    possesion_count: Res<PossesionCount>,
) {
    for _ in boss_death_event.iter() {
        // IDEA: a in game text could prompts the number of spectators used/sacrified
        // if #ofpossesion = #ofspectators, replace it by "with the last spectator mad enough to stay"
        println!(
            "CONGRATS ! You Killed the Hero with only {} spectators sacrifies",
            possesion_count.0
        );
        // IDEA: The final possesion... (see the bible (by olf))
    }
}
