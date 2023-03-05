//! All methods only linked to the detection Sensor/trigger Boss Behavior
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;

use crate::{
    characters::{
        aggression::{AttackCooldown, DeadBody},
        animations::CharacterState,
        movement::CharacterHitbox,
        player::{Player, PlayerHitbox},
    },
    collisions::CollisionEventExt,
    constants::character::boss::BOSS_SMASH_COOLDOWN,
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

// -- Actions --

/// Groups all Boss' Sensors
#[derive(Component, Deref, DerefMut)]
pub struct BossActions(pub Option<Vec<BossAction>>);

#[derive(Debug, Clone, Eq, Inspectable, PartialEq)]
pub enum BossAction {
    Wait(i32),
    // -- Attacks --
    Smash,
    FeintSmash,
    FallenAngel,
    LaserRain,
    // -- Movements --
    Dash,
    /// TP to x, should prefer the ²f32
    Tp(i32),
}

/// Indicates that the last actions is completed
/// DOC
pub struct ActionCompletedEvent;

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
    // &mut BossBehavior,
    mut boss_query: Query<(&mut BossActions, &Transform), (With<Boss>, Without<DeadBody>)>,
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
                    // mut boss_behavior
                    if let Ok((mut boss_actions, boss_transform)) = boss_query.get_single_mut() {
                        // The player is running away
                        if (boss_transform.translation.x < player_transform.translation.x
                            && player_vel.linvel.x < 0.)
                            || (boss_transform.translation.x > player_transform.translation.x
                                && player_vel.linvel.x > 0.)
                        {
                            // *boss_behavior = BossBehavior::DashInAttack;
                            // REFACTOR: prefer send a event to modify that
                            match boss_actions.0 {
                                Some(_) => continue,
                                None => {
                                    boss_actions.0 = Some(vec![BossAction::Dash, BossAction::Smash])
                                }
                            }
                        }
                    }
                }
                _ => continue,
            }
        }
    }
}

pub fn boss_actions_completed(
    mut action_completed_event: EventReader<ActionCompletedEvent>,

    mut boss_actions_query: Query<&mut BossActions, With<Boss>>,
) {
    for _ in action_completed_event.iter() {
        if let Ok(mut boss_actions) = boss_actions_query.get_single_mut() {
            match &boss_actions.0 {
                None => warn!("There is no actions for the boss, (None)"),
                Some(actions) => {
                    if let Some((_first, rem)) = actions.split_first() {
                        // pop first only
                        if rem.is_empty() {
                            boss_actions.0 = None;
                        } else {
                            // this change will be detected by the fn boss_actions()
                            boss_actions.0 = Some(rem.to_vec());
                        }
                    } else {
                        // shouldn't be the case
                        warn!("There is no actions for the boss, (Some(vec![])")
                    }
                }
            }
        }
    }
}

/// Executes the first order when the boss_actions changes
///
/// Catches changes made by the fn *boss_actions_completed*()
///
/// -------
///
/// ```markdown
/// If your system only runs sometimes (such as with states or run criteria),
/// you do ***not*** have to worry about missing changes.
/// ```
///
/// ^^^--- From [Change Detection](https://bevy-cheatbook.github.io/programming/change-detection.html)
///
/// -------
///
/// So there is no need for this system to runs just after the fn *boss_actions_completed*()
pub fn boss_actions(
    mut commands: Commands,
    mut boss_query: Query<
        (&BossActions, &Transform, &mut CharacterState),
        (With<Boss>, Changed<BossActions>, Without<DeadBody>),
    >,
    boss_proximity_sensor_query: Query<
        Entity,
        (
            With<Sensor>,
            With<BossSensor>,
            With<ProximitySensor>,
            Without<AttackCooldown>,
        ),
    >,
) {
    if let Ok((boss_actions, _boss_transform, mut boss_state)) = boss_query.get_single_mut() {
        match &boss_actions.0 {
            None => {}
            Some(actions) => {
                for action in actions.iter() {
                    match action {
                        BossAction::Dash => {
                            *boss_state = CharacterState::Dash;
                        }
                        BossAction::Wait(_time) => {
                            // add wait timer
                        }
                        BossAction::Smash => {
                            *boss_state = CharacterState::TransitionToCharge;
                            // REFACTOR: Cooldown management
                            if let Ok(attack_sensor) = boss_proximity_sensor_query.get_single() {
                                commands
                                    // REFACTOR: where the cooldown timer is placed
                                    .entity(attack_sensor) // **boss
                                    .insert(AttackCooldown(Timer::from_seconds(
                                        BOSS_SMASH_COOLDOWN,
                                        TimerMode::Once,
                                    )));
                            }
                        }
                        _ => {}
                    }
                    // just look the first action
                    break;
                }
            }
        }
    }
}
