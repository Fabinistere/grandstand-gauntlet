//! A big gouchi Boss

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    characters::{
        aggression::{AttackCooldown, AttackHitbox, AttackSensor, Hp},
        // Invulnerable,
        animations::CharacterState,
        movement::CharacterHitbox,
        player::PlayerHitbox,
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

/// Happens when:
///   - character::npcs::boss::boss_close_detection
///     - The player hitbox/Sensor enters the BossSensor
///   - character::npcs::boss::???
///     - The player hitbox/Sensor is still in the BossSensor
///     They must be punished by sanding their bones.
///
/// Read in
///   - character::npcs::boss::aggression::boss_attack_event_handler
///     - Launch an attack to the current facing direction
///     (cause they always look at the player)
///
/// # Note
///
/// Add target_entity: only if the wanted direction is different from current
pub struct BossAttackEvent {
    attacker_entity: Entity,
}

#[derive(Component)]
pub struct BossSensor;

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
///
/// And we could just deactivate the player hitbox if Invulnerable by removing `ActiveEvents`
///
/// ^^^----- I don't know if that could just work: (if not) for more depts: [Collision groups and solver groups](https://rapier.rs/docs/user_guides/bevy_plugin/colliders/#collision-groups-and-solver-groups)
pub fn boss_close_detection(
    mut commands: Commands,

    // mut collision_events: EventReader<CollisionEvent>,
    rapier_context: Res<RapierContext>,

    boss_attack_sensor_query: Query<
        (Entity, &Parent),
        (With<Sensor>, With<BossSensor>, Without<AttackCooldown>),
    >,
    player_sensor_query: Query<(Entity, &Parent), (With<PlayerHitbox>, With<CharacterHitbox>)>,

    mut boss_attack_event: EventWriter<BossAttackEvent>,
) {
    // Phase 1 - Sensor
    if let Ok((attack_sensor, boss)) = boss_attack_sensor_query.get_single() {
        if let Ok((player_sensor, _player)) = player_sensor_query.get_single() {
            // Phase 3 - Player TP proof
            if rapier_context.intersection_pair(attack_sensor, player_sensor) == Some(true) {
                // IDEA: MUST-HAVE - Disable turn/movement when the boss attack (avoid spinning attack when passing behind the boss)
                // ^^^^^------ With Dash/Death TP for example

                boss_attack_event.send(BossAttackEvent {
                    attacker_entity: **boss,
                });

                // Phase 2 - Timer
                commands
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

pub fn boss_attack_event_handler(
    mut boss_attack_event: EventReader<BossAttackEvent>,
    // If needed to check the Player Invulnerability state:
    // player_query: Query<Entity, (With<Player>, Without<Invulnerable>)>,
    mut attacker_query: Query<&mut CharacterState>,
) {
    for BossAttackEvent { attacker_entity } in boss_attack_event.iter() {
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
                        match (
                            smash_hitbox_query.get(*hitbox_child),
                            fallen_angel_hitbox_query.get(*hitbox_child),
                        ) {
                            (Ok(Smash), Err(_)) => {
                                if *character_state == CharacterState::Attack {
                                    // info!("DEBUG: Smash Active Inserted on {}", _name);
                                    commands
                                        .entity(Smash)
                                        .insert(ActiveEvents::COLLISION_EVENTS);
                                } else {
                                    // info!("DEBUG: Smash Active Removed on {}", _name);
                                    commands.entity(Smash).remove::<ActiveEvents>();
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
                            (Err(_), Err(_)) => warn!("Non Indexed Attack (Smash or Fallen Angel)"),
                            (Ok(_), Ok(_)) => {
                                warn!("Attack indexed twice (Smash and Fallen Angel)")
                            }
                        }
                        // match attack_hitbox_query.get(*child)
                    }
                }
            }
        }
    }
}
