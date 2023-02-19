use bevy::prelude::*;

use super::{npcs::boss::Boss, player::Player};

pub struct AggressionPlugin;

impl Plugin for AggressionPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app 
            .add_event::<FlipAttackSensor>()
            .add_system(flip_attack_sensor)
            .add_system(charged_attack)
            ;
    }
}

#[derive(Component)]
pub struct AttackSensor;

#[derive(Component, Debug)]
pub struct AttackCharge {
    pub charging: bool,
    pub timer: Timer,
}

/// DOC
pub struct FlipAttackSensor(pub Entity);

fn charged_attack(time: Res<Time>, mut query: Query<&mut AttackCharge>) {
    for mut attack_charge in query.iter_mut() {
        if attack_charge.charging {
            attack_charge.timer.tick(time.delta());
        }
    }
}

fn flip_attack_sensor(
    mut flip_direction_event: EventReader<FlipAttackSensor>,

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
