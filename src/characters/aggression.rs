use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    // collisions::CollisionEventExt,
    characters::{
        animations::CharacterState,
        movement::CharacterHitbox,
        npcs::boss::Boss,
        player::Player,
    }, soul_shift::{SoulShiftEvent, SoulShifting}, crowd::CrowdMember
};

pub struct AggressionPlugin;

impl Plugin for AggressionPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app 
            // -- Aesthetic --
            .add_event::<FlipAttackSensorEvent>()
            .add_system(flip_attack_sensor)
            .add_system(damage_animation.label("Damage Animation").after("Damage Hit"))
            // -- ? --
            .add_event::<DamageHitEvent>()
            .add_system(invulnerability_timer.label("Invulnerability Timer"))
            .add_system(cooldown_timer.label("Cooldown Timer"))
            .add_system(attack_hitbox_activation.label("Attack Hitbox Activation"))
            .add_system(attack_collision.label("Attack Collision").after("Attack Hitbox Activation"))
            .add_system(damage_hit.label("Damage Hit").after("Attack Collision"))
            ;
    }
}

#[derive(Component)]
pub struct AttackSensor;

/// Contains the damage it deals
#[derive(Component)]
pub struct AttackHitbox(pub i32);

#[derive(Component, Deref, DerefMut)]
pub struct Invulnerable(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct AttackCooldown(pub Timer);

#[derive(Component)]
pub struct DeadBody;

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
///   - characters::aggression::attack_collision
///     - The target got hit by a attack hitbox
///     which is not their attack (no self-harm)
/// 
/// Read in
///   - characters::aggression::damage_hit
///     - Target gets reckted (lower hp).
///     Death if it was too much
pub struct DamageHitEvent {
    /// Where the damage is stored
    pub attack_hitbox: Entity,
    /// Entity getting hit
    pub target: Entity,
}

/// Happens when
///   - characters::player::player_movement
///     - The player turns into opposite direction
///     So do their hitbox.
/// 
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

/// Lower the cooldown timer and remove it when it fishied
fn invulnerability_timer(
    mut commands: Commands,
    time: Res<Time>,
    
    // The Boss has their cooldown in their Attack Range Sensor
    mut invulnerable_character: Query<(Entity, &mut Invulnerable)>,
) {
    for (character, mut invulnerability) in invulnerable_character.iter_mut() {
        invulnerability.tick(time.delta());

        if invulnerability.just_finished() {
            commands
                .entity(character)
                .remove::<Invulnerable>();
        }
    }
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
                        // OPTIMIZE: Hitbox Activation - This statement will be called a bit too much
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
                        // match attack_hitbox_query.get(*child)
                    }
                }
            }
            
        }
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
fn attack_collision(
    // mut collision_events: EventReader<CollisionEvent>,
    rapier_context: Res<RapierContext>,
    
    attack_hitbox_query: Query<(Entity, &Parent), (With<Sensor>, With<AttackHitbox>, With<ActiveEvents>)>,
    character_hitbox_query: Query<(Entity, &Parent), With<CharacterHitbox>>,
    
    // vv-- They has as child a attackHitbox which inherit their transform
    attack_sensor_query: Query<(Entity, &Parent), With<AttackSensor>>,
    
    mut damage_hit_event: EventWriter<DamageHitEvent>,
) {
    // OPTIMIZE: Querying all attack hitbox then all character hitbox is not very efficient
    for (attack_hitbox_entity, parent_hitbox) in attack_hitbox_query.iter() {
        for (character_hitbox, target) in character_hitbox_query.iter() {
            if rapier_context.intersection_pair(attack_hitbox_entity, character_hitbox) == Some(true) {
                match attack_sensor_query.get(**parent_hitbox) {
                    Err(e) => warn!("The attackHitbox's hierarchy is invalid: {:?}", e),
                    Ok((_, attacker)) => {
                        if **attacker != **target {
                            damage_hit_event.send(DamageHitEvent {
                                attack_hitbox: attack_hitbox_entity,
                                target: **target
                            });
                        }
                    }
                }

            }
        }
    }
}

/// DOC
/// 
/// Send a Death Event if it's too much...
fn damage_hit(
    mut damage_hit_event: EventReader<DamageHitEvent>,
    
    mut commands: Commands,
    
    // With<ActiveEvents>
    attack_hitbox_query: Query<&AttackHitbox, With<Sensor>>,
    mut target_query: Query<&mut Hp, (Without<Invulnerable>, Without<SoulShifting>, Without<CrowdMember>)>,
    
    mut soul_shift_event: EventWriter<SoulShiftEvent>,
) {
    for DamageHitEvent {attack_hitbox, target} in damage_hit_event.iter() {
        // There is much of it ----vvvv
        // info!("Damage Hit Event !");
        match (attack_hitbox_query.get(*attack_hitbox), target_query.get_mut(*target)) {
            // Invulnerable target
            (Ok(_),Err(_)) => continue,
            // Invalid Attacker
            (Err(e),_) => warn!("Problem {:?}", e),
            (Ok(attack_damage), Ok(mut hp)) => {
                if hp.current <= attack_damage.0 {
                    hp.current = 0;
                    
                    // TODO: send Player Death Event when the player die
                    // atm all dying entity will trigger the soul shift/kill the player
                    commands.entity(*target).insert(SoulShifting);
                    soul_shift_event.send(SoulShiftEvent);
                    // TODO: Boss Death Event
                } else {
                    hp.current -= attack_damage.0;
                    // TODO: Seperate player and boss gestion of getting hit
                    // TODO: Invulnerable Hint
                    commands
                        .entity(*target)
                        .insert(Invulnerable(Timer::from_seconds(2., TimerMode::Once)));
                }
            }
        }
    }   
}

/// Change the Animation to Hit when being hurted.
/// TODO: Prevent hit anim while healing
/// Carefull: Even if the Hp is rising this animation will trigger
fn damage_animation(
    // DEBUG: Crowd getting hit (maybe the prb is here)
    mut bleeding_character_query: Query<(Entity, &mut CharacterState), Changed<Hp>>,
) {
    for (_hurted_character, mut character_state) in bleeding_character_query.iter_mut() {
        *character_state = CharacterState::Hit;
    }
}
