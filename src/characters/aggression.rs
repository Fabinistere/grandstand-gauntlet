use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::collisions::CollisionEventExt;

use super::{npcs::boss::Boss, player::Player, movement::CharacterHitbox};

pub struct AggressionPlugin;

impl Plugin for AggressionPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app 
            .add_event::<FlipAttackSensorEvent>()
            .add_system(flip_attack_sensor)
            .add_system(damage_hit)
            ;
    }
}

#[derive(Component)]
pub struct AttackSensor;

/// Contains the damage it deals
#[derive(Component)]
pub struct AttackHitbox(pub i32);

#[derive(Component)]
pub struct Invulnerable;

#[derive(Component)]
pub struct Hp {
    pub current: i32,
    pub max: i32,
}

impl Hp {
    pub fn default() -> Hp {
        Hp { current: 100, max: 100 }
    }

    pub fn new(max: i32) -> Hp {
        Hp { current: max, max }
    }
}

/// Happens when
///   - characters::player::player_movement
///     - The player turns into opposite direction
///     So do their hitbox.
/// Read in
///   - characters::aggression::flip_attack_sensor
///     - Flip their attack sensor vertically
///     (avoid having one hitbox for the right and one for the left)
pub struct FlipAttackSensorEvent(pub Entity);

fn flip_attack_sensor(
    mut flip_direction_event: EventReader<FlipAttackSensorEvent>,

    character_query: Query<&Children, Or<(With<Player>, With<Boss>)>>,
    // With<Sensor>, 
    mut attack_sensor_query: Query<&mut Transform, With<AttackSensor>>,
) {
    for event in flip_direction_event.iter() {
        match character_query.get(event.0) {
            Err(e) => warn!("can't flip the attack of this entity: {:?}", e),
            Ok(children) => {
                for child in children.iter() {
                    match attack_sensor_query.get_mut(*child) {
                        Err(_) => continue,
                        // attack_sensor_transform
                        // flip the sensor vertically
                        Ok(mut transform) => {
                            transform.translation.x *= -1.;
                            // transform.scale.x *= -1.;
                        }
                    }
                }
            }
        }
    }
}

/// Inflicts Damage (contains within the attack hitbox) to the touched entity.
/// 
/// Send a Death Event if it's too much...
fn damage_hit(
    mut collision_events: EventReader<CollisionEvent>,
    rapier_context: Res<RapierContext>,
    
    attack_hitbox_query: Query<(&AttackHitbox, &Parent), With<Sensor>>,
    character_hitbox_query: Query<(Entity, &Parent), With<CharacterHitbox>>,

    mut target_query: Query<&mut Hp, Without<Invulnerable>>,
) {
    for collision_event in collision_events.iter() {
        let entity_1 = collision_event.entities().0;
        let entity_2 = collision_event.entities().1;

        // info!("DEBUG: {:?} x {:?}", entity_1, entity_2);

        if rapier_context.intersection_pair(entity_1, entity_2) == Some(true) {
            match (attack_hitbox_query.get(entity_1), attack_hitbox_query.get(entity_2), character_hitbox_query.get(entity_1), character_hitbox_query.get(entity_2)) {
                (Ok((attack_hitbox, _attacker)), Err(_), Err(_), Ok((_character_hitbox, target)))
                | (Err(_), Ok((attack_hitbox, _attacker)), Ok((_character_hitbox, target)), Err(_)) => {
                    match target_query.get_mut(**target) {
                        Err(e) => warn!("No HP Component in the targeted entity: {:?}", e),
                        Ok(mut hp) => {
                            if hp.current <= attack_hitbox.0 {
                                hp.current = 0;
                                // TODO: send Death Event
                            } else {
                                hp.current -= attack_hitbox.0;
                            }
                        }
                    }
                }
                // There is no attack_hitbox/character_hitbox involved
                _ => continue,

            }
        }
    }
}
