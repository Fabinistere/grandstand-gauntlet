use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    characters::{
        aggression::FlipAttackSensorEvent,
        animations::CharacterState,
        movement::Speed,
        player::{Player, PlayerHitbox},
    },
    collisions::CollisionEventExt,
    crowd::CrowdMember,
};

use super::{Boss, BossMovementSensor, ChaseBehavior};

pub fn stare_player(
    mut boss_query: Query<(Entity, &mut TextureAtlasSprite, &Transform), With<Boss>>,
    player_query: Query<&Transform, (With<Player>, Without<CrowdMember>)>,
    mut flip_direction_event: EventWriter<FlipAttackSensorEvent>,
) {
    let (boss, mut boss_sprite, boss_transform) = boss_query.single_mut();
    let player_transform = player_query.single();

    if boss_sprite.flip_x != (boss_transform.translation.x > player_transform.translation.x) {
        flip_direction_event.send(FlipAttackSensorEvent(boss));
    }
    boss_sprite.flip_x = boss_transform.translation.x > player_transform.translation.x;
}

pub fn chase_player(
    mut boss_query: Query<
        (
            Entity,
            &mut CharacterState,
            &Transform,
            &Speed,
            &mut Velocity,
        ),
        (With<Boss>, With<ChaseBehavior>),
    >,
    player_query: Query<&Transform, (With<Player>, Without<CrowdMember>)>,
    time: Res<Time>,
) {
    if let Ok((_boss, mut boss_state, boss_transform, speed, mut boss_vel)) =
        boss_query.get_single_mut()
    {
        let player_transform = player_query.single();

        let direction = player_transform.translation;

        let left = direction.x < boss_transform.translation.x;
        let right = direction.x > boss_transform.translation.x;

        let close_range_width = boss_transform.scale.x * 10.;

        // The boss is in range with the player
        if direction.x - close_range_width < boss_transform.translation.x
            && direction.x + close_range_width > boss_transform.translation.x
        {
            boss_vel.linvel = Vect::ZERO;
            // TODO: New beahvior if in range
        } else {
            // The boss is away from the player

            let x_axis = -(left as i8) + right as i8;

            boss_vel.linvel.x = x_axis as f32 * **speed * 200. * time.delta_seconds();
        }

        // ---- Animation ----

        // if there is any movement
        if (left || right) && *boss_state != CharacterState::Run {
            *boss_state = CharacterState::Run;
        } else if !(left || right) && *boss_state == CharacterState::Run {
            *boss_state = CharacterState::Idle;
        }
    }
}

/// DOC: Naming with boss_close_detection or merge
///
/// For movement this one
pub fn close_range_detection(
    mut collision_events: EventReader<CollisionEvent>,

    mut commands: Commands,
    rapier_context: Res<RapierContext>,

    boss_personal_space_query: Query<(Entity, &Parent), (With<Sensor>, With<BossMovementSensor>)>,
    player_hitbox_query: Query<(Entity, &Parent), With<PlayerHitbox>>,

    mut boss_query: Query<
        (
            Entity,
            // modify speed to drift
            &Speed,
            // TODO: negate this when entering
            &mut Velocity,
        ),
        With<Boss>,
    >,
    player_query: Query<&Transform, (With<Player>, Without<CrowdMember>)>,
) {
    for collision_event in collision_events.iter() {
        let entity_1 = collision_event.entities().0;
        let entity_2 = collision_event.entities().1;

        // check if the sensor is a DetectionSensor
        match (
            boss_personal_space_query.get(entity_1),
            boss_personal_space_query.get(entity_2),
            player_hitbox_query.get(entity_1),
            player_hitbox_query.get(entity_2),
        ) {
            // only one of them contains DetectionSensor: detection_sensor
            // and the other one is a player_hitbox
            (
                Ok((_detection_sensor, b_parent)),
                Err(_e1),
                Err(_e2),
                Ok((_player_hitbox, p_parent)),
            )
            | (
                Err(_e1),
                Ok((_detection_sensor, b_parent)),
                Ok((_player_hitbox, p_parent)),
                Err(_e2),
            ) => {
                // DEBUG: info!(target: "Collision with a sensor and a player hitbox", "{:?} and {:?}", _detection_sensor, _player_hitbox);

                match (boss_query.get_mut(**b_parent), player_query.get(**p_parent)) {
                    (Ok((boss, _boss_speed, mut boss_vel)), Ok(_player_location)) => {
                        // just entered the personal space
                        if rapier_context.intersection_pair(entity_1, entity_2) == Some(true) {
                            // info!("DEBUG: Personal Space entered");
                            commands.entity(boss).remove::<ChaseBehavior>();
                            boss_vel.linvel = Vect::ZERO;
                        } else {
                            // info!("DEBUG: Personal Space exited");
                            commands.entity(boss).insert(ChaseBehavior);
                        }
                    }
                    _ => continue,
                }
            }
            _ => continue,
        }
    }
}
