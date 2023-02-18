use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    collisions::CollisionEventExt,
    characters::{
        animations::CharacterState,
        movement::CharacterHitbox,
        npcs::boss::Boss,
        player::Player,
    }
};

use super::player::PlayerHitbox;

pub struct AggressionPlugin;

impl Plugin for AggressionPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app 
            // -- Aesthetic --
            .add_event::<FlipAttackSensorEvent>()
            .add_system(flip_attack_sensor)
            .add_system(damage_animation)
            // -- ? --
            .add_system(damage_hit)
            .add_system(attack_hitbox_activation)
            .add_system(cooldown_timer)
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

#[derive(Component, Deref, DerefMut)]
pub struct AttackCooldown(pub Timer);

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

/// Change the Animation to Hit when being hurted.
/// TODO: Prevent hit anim while healing
/// Carefull: Even if the Hp is rising this animation will trigger
fn damage_animation(
    mut bleeding_character_query: Query<(Entity, &mut CharacterState), Changed<Hp>>,
) {
    for (_hurted_character, mut character_state) in bleeding_character_query.iter_mut() {
        *character_state = CharacterState::Hit;
    }
}

/// Lower the cooldown timer and remove it when it fishied
fn cooldown_timer(
    mut commands: Commands,
    time: Res<Time>,

    // The Boss has their cooldown in their Attack Range Sensor
    mut character_on_cooldown: Query<(Entity, &mut AttackCooldown)>,
) {
    for (character, mut cooldown) in character_on_cooldown.iter_mut() {
        cooldown.tick(time.delta());

        if cooldown.just_finished() {
            commands
                .entity(character)
                .remove::<AttackCooldown>();
        }
    }
}

/// Inflicts Damage (contains within the attack hitbox) to the touched entity.
/// 
/// Send a Death Event if it's too much...
fn damage_hit(
    mut collision_events: EventReader<CollisionEvent>,
    rapier_context: Res<RapierContext>,
    
    attack_hitbox_query: Query<(Entity, &AttackHitbox, &Parent), (With<Sensor>, With<ActiveEvents>)>,
    character_hitbox_query: Query<(Entity, &Parent), (With<PlayerHitbox>, With<CharacterHitbox>)>,

    mut target_query: Query<&mut Hp, Without<Invulnerable>>,
) {
    // REFACTOR: IF THE PLAYER attack in the same moment this if will be false (multiple entities)
    // and doen't work for the player attack
    if let Ok((attack_hitbox_entity, attack_hitbox, _attacker)) = attack_hitbox_query.get_single() {
        // TODO: Getting hit makes you invulnerable
        // ATM tou're getting OS
        if let Ok((character_hitbox, target)) = character_hitbox_query.get_single() {
            if rapier_context.intersection_pair(attack_hitbox_entity, character_hitbox) == Some(true) {
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
        } else { info!("No PlayerHitbox") }
    }
    // for collision_event in collision_events.iter() {
    //     let entity_1 = collision_event.entities().0;
    //     let entity_2 = collision_event.entities().1;

    //     // REFACTOR: This intersection_pair doesn't care about ActiveEvent
    //     // ^^^^^^^^^-------------- We need to query the attack_hitbox with ActiveEvent
    //     // BUG: This method only allow dmg when enter the hitbox while it's attacking
    //     if rapier_context.intersection_pair(entity_1, entity_2) == Some(true) {
    //         match (attack_hitbox_query.get(entity_1), attack_hitbox_query.get(entity_2), character_hitbox_query.get(entity_1), character_hitbox_query.get(entity_2)) {
    //             // A AtatckHitbox and a CharacterHitbox is involved
    //             (Ok((attack_hitbox, _attacker)), Err(_), Err(_), Ok((_character_hitbox, target)))
    //             | (Err(_), Ok((attack_hitbox, _attacker)), Ok((_character_hitbox, target)), Err(_)) => {
    //                 match target_query.get_mut(**target) {
    //                     Err(e) => warn!("No HP Component in the targeted entity: {:?}", e),
    //                     Ok(mut hp) => {
    //                         if hp.current <= attack_hitbox.0 {
    //                             hp.current = 0;
    //                             // TODO: send Death Event
    //                         } else {
    //                             hp.current -= attack_hitbox.0;
    //                         }
    //                     }
    //                 }
    //             }
    //             // There is no attack_hitbox/character_hitbox involved
    //             _ => continue,

    //         }
    //     }
    // }
}

/// Activate when the character is on animation phase Attack,
/// Deactivate else.
fn attack_hitbox_activation(
    mut commands: Commands,

    character_query: Query<
        (
            &CharacterState,
            &Children,
            &Name,
        ),
        (Changed<CharacterState>, Or<(With<Player>, With<Boss>)>),
    >,
    parent_hitbox_position_query: Query<(Entity, &Children), With<AttackSensor>>,
    // attack_hitbox_query: Query<Entity, (With<AttackHitbox>, With<Sensor>)>
) {
    for (character_state, children, _name) in character_query.iter() {
        // info!("DEBUG: {} Changed {:?}", name, *character_state);
        for child in children.iter() {
            match parent_hitbox_position_query.get(*child) {
                Err(_) => continue,
                // The parent Hitbox contains the modifiable transform
                // for all their hitbox children
                Ok((_parent_hitbox, hitbox_children)) => {
                    for hitbox_child in hitbox_children.iter() {
                        // OPTIMIZE: Hitbox Activation - This statement wil be called a bit too much
                        // vv-- to just see uncomment the two DEBUG info below --vv
                        if *character_state == CharacterState::Attack
                        || *character_state == CharacterState::SecondAttack
                        || *character_state == CharacterState::TransitionToCharge
                        {
                            // info!("DEBUG: Inserted on {}", _name);
                            commands.entity(*hitbox_child).insert(ActiveEvents::COLLISION_EVENTS);
                        } else {
                            // info!("DEBUG: Removed on {}", _name);
                            commands.entity(*hitbox_child).remove::<ActiveEvents>();
                        }
                        // match attack_hitbox_query.get(*child) {
                        //     Err(_) => continue,
                        //     Ok(attack_hitbox) => {
                        //         if *character_state == CharacterState::Attack
                        //         || *character_state == CharacterState::SecondAttack
                        //         || *character_state == CharacterState::TransitionToCharge
                        //         {
                        //             // info!("DEBUG: Inserted on {}", _name);
                        //             commands.entity(attack_hitbox).insert(ActiveEvents::COLLISION_EVENTS);
                        //         } else {
                        //             // info!("DEBUG: Removed on {}", _name);
                        //             commands.entity(attack_hitbox).remove::<ActiveEvents>();
                        //         }
                        //     }
                        // }
                    }
                }
            }
            
        }
    }
}
