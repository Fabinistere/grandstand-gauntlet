use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;

use crate::{
    // collisions::CollisionEventExt,
    characters::{
        animations::CharacterState,
        movement::CharacterHitbox,
        npcs::boss::{Boss, aggression::BossDeathEvent},
        player::Player,
    },
    soul_shift::{SoulShiftEvent, SoulShifting},
    crowd::CrowdMember,
    MySystems
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
            .add_system(
                damage_animation
                    .label(MySystems::DamageAnimation)
                    .after(MySystems::DamageHit)
            )
            // -- ? --
            .add_event::<DamageHitEvent>()
            .add_system(invulnerability_timer)
            .add_system(cooldown_timer)
            .add_system(
                player_attack_hitbox_activation
                    .label(MySystems::PlayerAttackHitboxActivation)
            )
            .add_system(
                attack_collision
                    .label(MySystems::AttackCollision)
                    .after(MySystems::PlayerAttackHitboxActivation)
                    .after(MySystems::BossAttackHitboxActivation)
            )
            .add_system(bam_the_player)
            .add_system(
                damage_hit
                    .label(MySystems::DamageHit)
                    .before(MySystems::SoulShift)
                    .after(bam_the_player)
                    .after(MySystems::AttackCollision)
            )
            ;
    }
}

// -- Character Description --

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

#[derive(Component, Debug)]
pub struct AttackCharge {
    pub charging: bool,
    pub timer: Timer,
}

#[derive(Component, Debug, Deref, DerefMut)]
pub struct Invulnerable(pub Timer);

#[derive(Component)]
pub struct DeadBody;

// -- Specific Hitbox for Attack --

/// Contains the damage it deals
/// 
/// # Note
/// 
/// TODO: Remove field and impl AttackStats instead.
#[derive(Component)]
pub struct AttackHitbox(pub i32);

/// Contains the damage it deals
/// 
/// # Note
/// 
/// TODO: Implement it
#[derive(Component)]
pub struct AttackStats {
    /// Static Damage
    pub base_damage: i32,
    /// 1.2 = 20% dmg up
    pub bonus: f32,
}

#[derive(Component)]
pub struct AttackSensor;

#[derive(Component, Deref, DerefMut)]
pub struct AttackCooldown(pub Timer);

// -- Events --

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
    for FlipAttackSensorEvent(character_to_flip) in flip_direction_event.iter() {
        match character_query.get(*character_to_flip) {
            Err(e) => warn!("can't flip the attack of this entity: {:?}", e),
            Ok(children) => {
                for child in children.iter() {
                    match attack_sensor_query.get_mut(*child) {
                        Err(_) => continue,
                        // attack_sensor_transform
                        // flip the sensor vertically
                        Ok(mut transform) => {
                            transform.translation.x *= -1.;
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
    mut invulnerable_character: Query<(Entity, &mut Invulnerable, &mut TextureAtlasSprite)>,
) {
    for (character, mut invulnerability, mut sprite) in invulnerable_character.iter_mut() {
        // eprintln!("{:#?}",invulnerability);
        invulnerability.tick(time.delta());

        // OPTIMIZE: The color change just need to change once per sec (not every frame)
        const WHITE: Color = Color::rgb(1.0, 1.0, 1.0);
        // change it to be custom (character depending)
        // 246 215 215
        const HIT_COLOR: Color = Color::rgb(0.96, 0.84, 0.84);
        
        if invulnerability.just_finished() {
            sprite.color = WHITE;
            commands
                .entity(character)
                .remove::<Invulnerable>();
        } else {
            // if the integer part of the timer left is odd
            sprite.color = if (invulnerability.elapsed_secs() as i32).rem_euclid(2) == 1 {
                HIT_COLOR
            } else {
                WHITE
            };
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
    // OPTIMIZE: Find a way to detect ongoing collision (even without movement)
    for (character_hitbox, target) in character_hitbox_query.iter() {
        match target_query.get(**target) {
            // The target is invulnerable
            Err(_) => continue,
            Ok(_) => {
                for (attack_hitbox_entity, parent_hitbox) in attack_hitbox_query.iter() {
                    if rapier_context.intersection_pair(attack_hitbox_entity, character_hitbox) == Some(true) {
                        match attack_sensor_query.get(**parent_hitbox) {
                            Err(e) => warn!("The attackHitbox's hierarchy is invalid: {:?}", e),
                            Ok((_, attacker)) => {
                                if **attacker != **target {

                                    // BUG: Send one too many undesirable/unsafe events - cause = for loop in attack_collision
                                    // info!("Damage Hit Event SENDED!");
                                    damage_hit_event.send(DamageHitEvent {
                                        attack_hitbox: attack_hitbox_entity,
                                        target: **target
                                    });
                                    // prevent other active hitbox to strike too
                                    return;
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
    boss_query: Query<Entity, With<Boss>>,

    mut soul_shift_event: EventWriter<SoulShiftEvent>,
    mut boss_death_event: EventWriter<BossDeathEvent>,
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
                    match (player_query.get(*target), boss_query.get(*target)) {
                        // BUG: this case happens cause of: for loop in attack_collision
                        (Err(_), Err(_)) => continue, // warn!("An entity that is neither a player nor a boss (probably a DeadBody) is molested.")
                        (Ok(_), Ok(_)) => warn!("The player is the boss and is under attack"),
                        (Err(_), Ok(_)) => {
                            hp.current = 0;
                            info!("Lethal Damage!");
        
                            boss_death_event.send(BossDeathEvent);
                        }
                        (Ok(_), Err(_)) => {
                            hp.current = 0;
                            info!("Lethal Damage!");
        
                            // commands.entity(*target).insert(SoulShifting);
                            soul_shift_event.send(SoulShiftEvent(*target));
                        }
                    }
                } else {
                    hp.current -= attack_damage.0;
                    // Seperate player and boss gestion of getting hit
                    let invul_timer;
                    match player_query.get(*target) {
                        // if not a player (normally is a boss)
                        Err(_) => {
                            invul_timer = 0.5;
                        }
                        Ok(_) => {
                            invul_timer = 2.;
                        }
                    }
                    
                    // IDEA: Invulnerable Hint
                    commands
                        .entity(*target)
                        .insert(Invulnerable(Timer::from_seconds(invul_timer, TimerMode::Once)));
                }
            }
        }   
    }
}

/// Change the Animation to Hit when being hurted.
/// 
/// # Note
/// 
/// TODO: feature - Prevent hit anim while healing
/// ^^^^^---- Carefull: Even if the Hp is rising this animation will trigger
/// BUG: Player can cancel the incomming attack by preshot the boss
pub fn damage_animation(
    // DEBUG: Crowd getting hit (maybe the prb is here)
    mut bleeding_character_query: Query<(Entity, &mut CharacterState), Changed<Hp>>,
) {
    for (_hurted_character, mut character_state) in bleeding_character_query.iter_mut() {
        *character_state = CharacterState::Hit;
    }
}
