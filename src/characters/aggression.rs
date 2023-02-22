use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;

use crate::{
    // collisions::CollisionEventExt,
    characters::{
        animations::CharacterState,
        movement::CharacterHitbox,
        npcs::boss::Boss,
        player::Player,
    }, soul_shift::{SoulShiftEvent, SoulShifting, start_soul_shift}, crowd::CrowdMember
};

use super::npcs::boss::BossAttack;

pub struct AggressionPlugin;

impl Plugin for AggressionPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app 
            // -- Aesthetic --
            .add_event::<FlipAttackSensorEvent>()
            .add_system(flip_attack_sensor)
            .add_system(charged_attack)
            .add_system(damage_animation.label("Damage Animation").after("Damage Hit"))
            // -- ? --
            .add_event::<DamageHitEvent>()
            .add_system(invulnerability_timer.label("Invulnerability Timer"))
            .add_system(cooldown_timer.label("Cooldown Timer"))
            .add_system(player_attack_hitbox_activation.label("Player Attack Hitbox Activation"))
            .add_system(
                attack_collision
                    .label("Attack Collision")
                    .after("Player Attack Hitbox Activation")
                    .after("Boss Attack Hitbox Activation")
                    .after(start_soul_shift)
            )
            .add_system(bam_the_player.label("Bam The Player"))
            .add_system(
                damage_hit
                    .label("Damage Hit")
                    .after(start_soul_shift)
                    .after("Bam The Player")
                    .after("Attack Collision")
            )
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

pub struct FlipAttackSensor(pub Entity);

/// Contains the damage it deals
#[derive(Component)]
pub struct AttackHitbox(pub i32);

#[derive(Component, Debug, Deref, DerefMut)]
pub struct Invulnerable(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct AttackCooldown(pub Timer);

#[derive(Component)]
pub struct DeadBody;

#[derive(Component, Inspectable)]
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

fn charged_attack(time: Res<Time>, mut query: Query<&mut AttackCharge>) {
    for mut attack_charge in query.iter_mut() {
        if attack_charge.charging {
            attack_charge.timer.tick(time.delta());
        }
    }
}

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

/// Activate when the character is on animation phase Attack,
/// Deactivate else.
fn player_attack_hitbox_activation(
    mut commands: Commands,
    
    player_query: Query<
        (
            &CharacterState,
            &Children,
            &Name,
        ),
        (Changed<CharacterState>, With<Player>),
    >,
    parent_hitbox_position_query: Query<(Entity, &Children), With<AttackSensor>>,
    // attack_hitbox_query: Query<Entity, (With<AttackHitbox>, With<Sensor>)>
) {
    for (character_state, children, _name) in player_query.iter() {
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
                        || *character_state == CharacterState::ChargedAttack
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


fn bam_the_player(
    keyboard_input: Res<Input<KeyCode>>,
    player_query: Query<Entity, (With<Player>, Without<Invulnerable>)>,
    boss_attack_hitbox: Query<Entity, (With<BossAttack>, With<AttackHitbox>)>,

    mut damage_hit_event: EventWriter<DamageHitEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Ok(player) = player_query.get_single() {
            info!("Bam dans ta gueule !");
            for attack_hitbox in boss_attack_hitbox.iter() {
                damage_hit_event.send(DamageHitEvent {
                    attack_hitbox,
                    target: player
                });
                break;
            }
        }
    }
}

// REFACTOR: ALL AGGRESSION COLLISION

/// Detected non self-harm touches.
fn attack_collision(
    // mut collision_events: EventReader<CollisionEvent>,
    rapier_context: Res<RapierContext>,
    
    attack_hitbox_query: Query<(Entity, &Parent), (With<Sensor>, With<AttackHitbox>, With<ActiveEvents>)>,
    character_hitbox_query: Query<(Entity, &Parent), With<CharacterHitbox>>,

    target_query: Query<Entity, (Without<Invulnerable>, Without<SoulShifting>)>,
    
    // vv-- They have as a child a attackHitbox which inherit their transform
    attack_sensor_query: Query<(Entity, &Parent), With<AttackSensor>>,

    mut damage_hit_event: EventWriter<DamageHitEvent>,
) {
    // OPTIMIZE: Querying all attack hitbox then all character hitbox is not very efficient
    for (attack_hitbox_entity, parent_hitbox) in attack_hitbox_query.iter() {
        for (character_hitbox, target) in character_hitbox_query.iter() {
            match target_query.get(**target) {
                // The target is invulnerable
                Err(_) => continue,
                Ok(_) => {
                    if rapier_context.intersection_pair(attack_hitbox_entity, character_hitbox) == Some(true) {
                        match attack_sensor_query.get(**parent_hitbox) {
                            Err(e) => warn!("The attackHitbox's hierarchy is invalid: {:?}", e),
                            Ok((_, attacker)) => {
                                if **attacker != **target {
                                    damage_hit_event.send(DamageHitEvent {
                                        attack_hitbox: attack_hitbox_entity,
                                        target: **target
                                    });
                                    // info!("Damage Hit Event !");
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Inflicts Damage (contains within the attack hitbox) to the touched entity.
/// 
/// Send a ~~Death Event~~ Soul Shift Event if it's too much...
/// 
/// # Note
fn damage_hit(
    mut damage_hit_event: EventReader<DamageHitEvent>,
    
    mut commands: Commands,
    
    // With<ActiveEvents>
    attack_hitbox_query: Query<&AttackHitbox, With<Sensor>>,
    mut target_query: Query<&mut Hp, (Without<Invulnerable>, Without<SoulShifting>, Without<CrowdMember>)>,
    
    player_query: Query<Entity, With<Player>>,

    mut soul_shift_event: EventWriter<SoulShiftEvent>,
) {
    for DamageHitEvent {attack_hitbox, target} in damage_hit_event.iter() {
        // There is no longer a long queue of events
        // info!("Damage Hit Event !");
        match (attack_hitbox_query.get(*attack_hitbox), target_query.get_mut(*target)) {
            // Invulnerable or SoulShifting target
            (Ok(_),Err(_)) => continue,
            // Invalid Attacker
            (Err(e),_) => warn!("Problem {:?}", e),
            (Ok(attack_damage), Ok(mut hp)) => {

                // info!("Damage Hit Event To a Vulnerable target!");
                if hp.current <= attack_damage.0 {
                    hp.current = 0;
                    info!("Lethal Damage!");

                    match player_query.get(*target) {
                        Err(_) => {
                            // TODO: Boss Death Event
                        }
                        Ok(_) => {
                            // commands.entity(*target).insert(SoulShifting);
                            soul_shift_event.send(SoulShiftEvent(*target));
                        }
                    }
                } else {
                    hp.current -= attack_damage.0;
                    // Seperate player and boss gestion of getting hit
                    let invul_timer;
                    // IDEA: Invulnerable Hint
                    match player_query.get(*target) {
                        // if not a player (normally is a boss)
                        Err(_) => {
                            invul_timer = 0.5;
                        }
                        Ok(_) => {
                            invul_timer = 2.;
                        }
                    }

                    commands
                        .entity(*target)
                        .insert(Invulnerable(Timer::from_seconds(invul_timer, TimerMode::Once)));
                }
            }
        }   
    }
}

/// Change the Animation to Hit when being hurted.
/// TODO: feature - Prevent hit anim while healing
/// Carefull: Even if the Hp is rising this animation will trigger
fn damage_animation(
    // DEBUG: Crowd getting hit (maybe the prb is here)
    mut bleeding_character_query: Query<(Entity, &mut CharacterState), Changed<Hp>>,
) {
    for (_hurted_character, mut character_state) in bleeding_character_query.iter_mut() {
        *character_state = CharacterState::Hit;
    }
}
