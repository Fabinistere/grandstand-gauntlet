use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{npcs::boss::Boss, player::Player};

pub struct AggressionPlugin;

impl Plugin for AggressionPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app 
            .add_event::<FlipAttackSensor>()
            .add_system(flip_attack_sensor)
            ;
    }
}

#[derive(Component)]
pub struct AttackSensor;

/// DOC
pub struct FlipAttackSensor(pub Entity);

fn flip_attack_sensor(
    mut flip_direction_event: EventReader<FlipAttackSensor>,

    character_query: Query<&Children, Or<(With<Player>, With<Boss>)>>,
    // With<Sensor>, 
    mut attack_sensor_query: Query<&mut Transform, With<AttackSensor>>,
) {
    for event in flip_direction_event.iter() {
        // info!("alo");
        match character_query.get(event.0) {
            Err(e) => warn!("can't flip the attack of this entity: {:?}", e),
            Ok(children) => {
                for child in children.iter() {
                    match attack_sensor_query.get_mut(*child) {
                        Err(_) => continue,
                        // attack_sensor_transform
                        // flip the sensor vertically
                        Ok(mut transform) => {
                            // info!("DEBUG: Attack Flip");
                            transform.translation.x *= -1.;
                            // transform.scale.x *= -1.;
                        }
                    }
                }
            }
        }
    }
}
