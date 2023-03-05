//! A big gouchi Boss

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::characters::{
    aggression::{AttackCooldown, AttackHitbox, AttackSensor, DeadBody, Invulnerable},
    // Invulnerable,
    animations::CharacterState,
    movement::CharacterHitbox,
    player::{PlayerHitbox, PossesionCount},
};

use super::{
    behaviors::{BossAction, BossActions, BossBehavior, BossSensor, ProximitySensor},
    Boss, BossAttackFallenAngel, BossAttackSmash,
};

// pub struct AggressionBossPlugin;

// impl Plugin for AggressionBossPlugin {
//     #[rustfmt::skip]
//     fn build(&self, app: &mut App) {
//         app .add_event::<BossAttackEvent>()
//             .add_system(boss_proximity_attack)
//             .add_system(boss_attack_event_handler)
//             ;
//     }
// }

/// DOC: Event
///
/// Happens when
///   - ???
///     - action
///
/// Read in
///   - ???
///     - action
pub struct BossDeathEvent;

/// When the player enters the sensor
/// The boss (if in ChaseBehavior) starts to attack them
///
/// Send a Event to launch a attack when a entity enters the sensor
/// and insert a timer to limit the number of attack while still in the sensor.
///
/// Remove this timer when leaving the sensor
///
/// # Note
///
/// The boss should still attacks while the player is invulnerable;
/// And not depends on the `EventReader<CollisionEvent>` which only see enter and exit of a sensor.
///
/// For more depts: [Collision groups and solver groups](https://rapier.rs/docs/user_guides/bevy_plugin/colliders/#collision-groups-and-solver-groups)
pub fn boss_proximity_attack(
    rapier_context: Res<RapierContext>,

    boss_proximity_sensor_query: Query<
        (Entity, &Parent),
        (
            With<Sensor>,
            With<BossSensor>,
            With<ProximitySensor>,
            Without<AttackCooldown>,
        ),
    >,
    player_sensor_query: Query<(Entity, &Parent), (With<PlayerHitbox>, With<CharacterHitbox>)>,

    mut boss_query: Query<(&BossBehavior, &mut BossActions), (With<Boss>, Without<DeadBody>)>,
) {
    // Phase 1 - Sensor
    if let Ok((attack_sensor, boss)) = boss_proximity_sensor_query.get_single() {
        if let Ok((player_sensor, _player)) = player_sensor_query.get_single() {
            // Phase 3 - Player TP proof
            if rapier_context.intersection_pair(attack_sensor, player_sensor) == Some(true) {
                match boss_query.get_mut(**boss) {
                    // DEBUG: (in the start of the game) / Every time a entity spawns, log the name + current identifier
                    Err(e) => warn!("This entity: {:?} Cannot be animated: {:?}", **boss, e),
                    Ok((behavior, mut boss_actions)) => {
                        if *behavior == BossBehavior::Chase {
                            // Phase 2 - Timer
                            match boss_actions.0 {
                                Some(_) => {}
                                None => {
                                    boss_actions.0 = Some(vec![
                                        BossAction::Smash,
                                        // BossAction::Smash,
                                        // BossAction::Smash,
                                        // BossAction::Smash,
                                    ])
                                }
                            }
                        }
                    }
                }
            }
        }
        // else { info!("No playerHitbox") }
    }
}

/// Activate when the character is on animation phase Attack,
/// Deactivate else.
pub fn boss_attack_hitbox_activation(
    mut commands: Commands,

    boss_query: Query<
        (&CharacterState, &Children, &Name),
        (Changed<CharacterState>, With<Boss>, Without<DeadBody>),
    >,
    parent_hitbox_position_query: Query<(Entity, &Children), With<AttackSensor>>,

    // All Kind of Boss Attack
    smash_hitbox_query: Query<Entity, (With<AttackHitbox>, With<BossAttackSmash>, With<Sensor>)>,
    fallen_angel_hitbox_query: Query<
        Entity,
        (
            With<AttackHitbox>,
            With<BossAttackFallenAngel>,
            With<Sensor>,
        ),
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
    mut commands: Commands,
    mut boss_query: Query<
        (
            Entity,
            &mut Velocity,
            &mut CharacterState,
            &mut TextureAtlasSprite,
        ),
        (With<Boss>, Without<DeadBody>),
    >,
    possesion_count: Res<PossesionCount>,
) {
    for _ in boss_death_event.iter() {
        // IDEA: a in game text could prompts the number of spectators used/sacrified
        // if #ofpossesion = #ofspectators, replace it by "with the last spectator mad enough to stay"
        println!(
            "CONGRATS ! You Killed the Hero with only {} spectators sacrifies",
            possesion_count.0
        );

        if let Ok((boss, mut rb_vel, mut state, mut sprite)) = boss_query.get_single_mut() {
            // Death Anim
            *state = CharacterState::Dead;
            rb_vel.linvel = Vect::ZERO;
            commands
                .entity(boss)
                .insert((
                    DeadBody,
                    // AnimationTimer(Timer::from_seconds(FRAME_TIME, TimerMode::Once)),
                ))
                // .remove::<Boss>()
                .remove::<Invulnerable>();

            const WHITE: Color = Color::rgb(1.0, 1.0, 1.0);
            sprite.color = WHITE;

            // IDEA: The final possesion... (see the bible (by olf))
        }
    }
}
