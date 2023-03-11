use bevy::{prelude::*, utils::HashMap};
use bevy_rapier2d::prelude::*;

use crate::{
    camera::camera_follow,
    characters::{
        aggression::{
            AttackCharge, AttackHitbox, AttackSensor, DeadBody, FlipAttackSensorEvent, Hp,
            Invulnerable,
        },
        animations::{AnimationIndices, AnimationTimer, CharacterState},
        movement::{CharacterHitbox, HyperDashTimer, MovementBundle, Speed},
        Freeze,
    },
    constants::character::{player::*, FRAME_TIME},
    crowd::CrowdMember,
    soul_shift::SoulShifting,
    MySystems,
};

use super::movement::DashTimer;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app .add_event::<CreatePlayerEvent>()
            .add_event::<PlayerDeathEvent>()
            .insert_resource(PossesionCount(0))
            .add_startup_system(spawn_first_player)
            .add_system(
                create_player
                    .label(MySystems::NewBeginning)
                    .after(MySystems::SoulShift)
            )
            // -- Camera --
            .add_system(camera_follow.after(MySystems::NewBeginning))
            // -- Aggression --
            .add_system(
                player_death_event
                    .label(MySystems::PlayerDeath)
                    .before(MySystems::NewBeginning)
                    // prioritize the Death anim in front of the Hit anim
                    .after(MySystems::DamageAnimation)
                    .after(MySystems::SoulShift),
                )
            // TODO: move this system up (in characters)
            .add_system(clean_up_dead_bodies.after(MySystems::PlayerDeath).after(MySystems::BossDeath))
            // Post Update needed cause depends on the DeadBody Filter which is added during the Update State
            .add_system_to_stage(CoreStage::PostUpdate, player_attack)
            // -- Movement --
            .add_system_to_stage(CoreStage::PostUpdate, player_movement)
            .add_system(player_dash)
            ;
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Resource, Debug, Deref, DerefMut)]
pub struct PossesionCount(pub i32);

#[derive(Component)]
pub struct PlayerHitbox;

#[derive(Component)]
pub struct PlayerAttack;

#[derive(Debug, Deref, DerefMut)]
pub struct CreatePlayerEvent(pub Entity);

/// DOC
/// Happens when
///   - soul_shift::start_soul_shift
///     - Soul Shift Done
///
/// Read in
///   - characters::player::player_death_event
///     - Death Animation
///     - Soul Shift Event
pub struct PlayerDeathEvent(pub Entity);

/// # Note
///
/// TODO: Make the charge much more valuable than the spamming (bonus attack)
/// IDEA: Cancel a attack (feint if the boss can pary)
/// TODO: Forbid to attaack while the player is in anim (attacking), cancel/replace Attacks
/// ^^^^---- By checking the state is in it
fn player_attack(
    keyboard_input: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    mut player_query: Query<
        (
            Entity,
            &mut CharacterState,
            &mut Velocity,
            &mut AttackCharge,
        ),
        (With<Player>, Without<DeadBody>),
    >,
) {
    if let Ok((_player, mut state, mut rb_vel, mut attack_charge)) = player_query.get_single_mut() {
        if *state == CharacterState::Attack
            || *state == CharacterState::SecondAttack
            || *state == CharacterState::ChargedAttack
        // || *state == CharacterState::Dead
        {
            return;
        }
        if keyboard_input.just_pressed(KeyCode::Return) || buttons.just_pressed(MouseButton::Left) {
            attack_charge.charging = true;
            attack_charge.timer.reset();
            *state = CharacterState::TransitionToCharge;
        } else if keyboard_input.just_released(KeyCode::Return)
            || buttons.just_released(MouseButton::Left)
        {
            // the charge was not canceled (prevent being canceled but having in the 'buffer' a charged attack)
            if *state == CharacterState::TransitionToCharge || *state == CharacterState::Charge {
                *state = if attack_charge.timer.finished() {
                    CharacterState::ChargedAttack
                } else {
                    CharacterState::Attack
                };
            }

            rb_vel.linvel = Vect::ZERO;
            attack_charge.timer.reset();
        }
    }
}

/// # Note
///
/// REFACTOR: DeadBodies Handler - these events are kinda weak (atm we have to abuse the tricks around...)
fn player_death_event(
    mut death_event: EventReader<PlayerDeathEvent>,

    mut commands: Commands,

    mut possesion_count: ResMut<PossesionCount>,
    mut player_query: Query<
        (
            Entity,
            &mut Velocity,
            &mut CharacterState,
            &mut TextureAtlasSprite,
        ),
        Without<CrowdMember>,
    >,
) {
    for player_death in death_event.iter() {
        // info!("DEATH EVENNNT !!");
        match player_query.get_mut(player_death.0) {
            Err(e) => warn!("DEBUG: No player.... {:?}", e),
            Ok((player, mut rb_vel, mut state, mut sprite)) => {
                // Death Anim
                *state = CharacterState::Dead;
                rb_vel.linvel = Vect::ZERO;
                commands
                    .entity(player)
                    .insert((
                        // already inserted in the soul_shift fn
                        DeadBody,
                        Name::new(format!("DeadBody n°{}", possesion_count.0)),
                        // AnimationTimer(Timer::from_seconds(FRAME_TIME, TimerMode::Once)),
                    ))
                    .remove::<SoulShifting>()
                    .remove::<Player>()
                    .remove::<Invulnerable>();

                const WHITE: Color = Color::rgb(1.0, 1.0, 1.0);
                sprite.color = WHITE;

                // The list's growing...
                possesion_count.0 += 1;
            }
        }
    }
}

/// Remove the hitbox/sensor from all new DeadBodies.
fn clean_up_dead_bodies(
    mut commands: Commands,
    mut dead_body_query: Query<(Entity, &mut Velocity), Added<DeadBody>>,
) {
    for (dead_body, mut rb_vel) in dead_body_query.iter_mut() {
        commands.entity(dead_body).despawn_descendants();
        rb_vel.linvel = Vect::ZERO;
    }
}

/// # Note
///
/// From [Bevy-cheatbook](https://bevy-cheatbook.github.io/programming/stages.html):
///
/// ```text
/// If you have systems that need to rely on the actions that other systems have performed by using Commands,
/// and need to do so during the same frame,
/// placing those systems into separate stages is the only way to accomplish that.
/// ```
///
/// This system runs after player_death_event but depending of the player's component,
/// DeadBody.
/// The commands are perform at the end of a Stage.
/// These two systems being in the same stage, mess this order.
///
/// SOLUTION: Place the player_movement / player_attack in a stage after player_death_event (Commands execution)
fn player_movement(
    mut commands: Commands,

    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<
        (
            Entity,
            &Speed,
            &mut Velocity,
            &mut TextureAtlasSprite,
            &mut CharacterState,
        ),
        (With<Player>, Without<CrowdMember>, Without<DeadBody>),
    >,
    time: Res<Time>,

    mut flip_direction_event: EventWriter<FlipAttackSensorEvent>,
) {
    if let Ok((player, speed, mut rb_vel, mut texture_atlas_sprite, mut player_state)) =
        player_query.get_single_mut()
    {
        // If player is attacking, don't allow them to move
        if *player_state == CharacterState::TransitionToCharge
            || *player_state == CharacterState::Charge
            || *player_state == CharacterState::Attack
            || *player_state == CharacterState::SecondAttack
            || *player_state == CharacterState::ChargedAttack
        // Order Execution of System (and stage) is taken care of the case a DeadBody wants to move/attack
        // || *player_state == CharacterState::Dead
        {
            rb_vel.linvel = Vect::ZERO;
            commands.entity(player).insert(Freeze);
            return;
        }

        // REFACTOR: Freeze component to tell to stop parallax movement
        commands.entity(player).remove::<Freeze>();

        let left = keyboard_input.pressed(KeyCode::Q)
            || keyboard_input.pressed(KeyCode::Left)
            || keyboard_input.pressed(KeyCode::A);
        let right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);

        let x_axis = (right as i8) - left as i8;

        rb_vel.linvel.x = x_axis as f32 * **speed * 200. * time.delta_seconds();

        // ---- Animation ----

        // if there is any movement
        if (left || right) && *player_state != CharacterState::Run {
            *player_state = CharacterState::Run;
        } else if !(left || right) && *player_state == CharacterState::Run {
            *player_state = CharacterState::Idle;
        }

        // ---- Direction ----

        if !(left && right) {
            if (left && !texture_atlas_sprite.flip_x) || (right && texture_atlas_sprite.flip_x) {
                flip_direction_event.send(FlipAttackSensorEvent(player));
            }

            // look where they are going - in the direction
            if right {
                texture_atlas_sprite.flip_x = false;
            } else if left {
                texture_atlas_sprite.flip_x = true;
            }
        }
    }
}

pub fn player_dash(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    player_query: Query<Entity, (With<Player>, Without<DashTimer>)>,
) {
    if let Ok(player) = player_query.get_single() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            info!("DAAASH !!");
            commands.entity(player).insert((
                DashTimer(Timer::from_seconds(0.2, TimerMode::Once)),
                Invulnerable(Timer::from_seconds(0.4, TimerMode::Once)),
            ));
        }
        // cost: 10hp (cause of bam_the_player)
        else if keyboard_input.just_pressed(KeyCode::F) {
            info!("HYPEEEEEEEEEER DAAASH !!");
            commands.entity(player).insert((
                HyperDashTimer(Timer::from_seconds(0.5, TimerMode::Once)),
                Invulnerable(Timer::from_seconds(0.5, TimerMode::Once)),
            ));
        }
    }
}

fn spawn_first_player(
    mut commands: Commands,
    mut create_player_event: EventWriter<CreatePlayerEvent>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("textures/character/character_spritesheet_v2.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(200., 200.), 35, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let texture_atlas_sprite = TextureAtlasSprite::new(0);

    let player_entity = commands
        .spawn((
            Player,
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: texture_atlas_sprite,
                transform: Transform::from_translation(PLAYER_POSITION.into()),
                ..default()
            },
        ))
        .id();
    create_player_event.send(CreatePlayerEvent(player_entity));
}

fn create_player(
    mut create_player_event: EventReader<CreatePlayerEvent>,
    mut commands: Commands,
    // mut transform_query: Query<&mut Transform>,
) {
    for CreatePlayerEvent(entity) in create_player_event.iter() {
        let mut animation_indices = AnimationIndices(HashMap::new());
        animation_indices.insert(CharacterState::Idle, PLAYER_IDLE_FRAMES);
        animation_indices.insert(CharacterState::Run, PLAYER_RUN_FRAMES);
        animation_indices.insert(
            CharacterState::TransitionToCharge,
            PLAYER_TRANSITION_TO_CHARGE_FRAMES,
        );
        animation_indices.insert(CharacterState::Charge, PLAYER_CHARGE_FRAMES);
        animation_indices.insert(CharacterState::Attack, PLAYER_FIRST_ATTACK_FRAMES);
        animation_indices.insert(CharacterState::SecondAttack, PLAYER_SECOND_ATTACK_FRAMES);
        animation_indices.insert(CharacterState::ChargedAttack, PLAYER_CHARGED_ATTACK_FRAMES);
        animation_indices.insert(CharacterState::Hit, PLAYER_HIT_FRAMES);
        animation_indices.insert(CharacterState::Dead, PLAYER_DEAD_FRAMES);

        // match transform_query.get_mut(*entity) {
        //     Err(e) => warn!("No transform in the entity, wat the freak: {:?}", e),
        //     Ok(mut transform) => {
        //         let mut new_position = CHAR_POSITION;
        //         // previous x
        //         new_position.2 = transform.translation.x;
        //         *transform = Transform::from_translation(new_position.into());
        //     }
        // }

        commands
            .entity(*entity)
            .insert((
                // Need to reinsert Player
                // when Soul Shifting to a new body
                Player,
                Name::new("Player"),
                // -- Animation --
                AnimationTimer(Timer::from_seconds(FRAME_TIME, TimerMode::Repeating)),
                animation_indices,
                CharacterState::default(),
                // -- Combat --
                // Hp::default(),
                Hp::new(PLAYER_HP),
                Invulnerable(Timer::from_seconds(10., TimerMode::Once)),
                // -- Hitbox --
                RigidBody::Dynamic,
                LockedAxes::ROTATION_LOCKED,
                MovementBundle {
                    speed: Speed::new(PLAYER_SPEED),
                    velocity: Velocity {
                        linvel: Vect::ZERO,
                        angvel: 0.,
                    },
                },
                // -- Attack --
                AttackCharge {
                    charging: false,
                    timer: Timer::from_seconds(CHARGED_ATTACK_HOLD, TimerMode::Once),
                },
            ))
            .remove::<SoulShifting>()
            .with_children(|parent| {
                // -- Player Hitbox And Sensor --
                // TODO: seperate the player Sensor to the player hitbox
                // ^^^^^-------- Sensor that will trigger the boss attack
                // And Hitbox which designates where is it precisely --^^
                parent.spawn((
                    Collider::ball(PLAYER_HITBOX_SIZE),
                    Transform::from_translation(PLAYER_HITBOX_OFFSET_Y.into()),
                    PlayerHitbox,
                    CharacterHitbox,
                    Sensor,
                    // ActiveEvents::COLLISION_EVENTS,
                    Name::new("Player Hitbox"),
                ));

                // -- Attack Hitbox --
                parent
                    .spawn((
                        SpatialBundle {
                            transform: Transform::from_translation(BOTTOM_WHIP_POS.into()),
                            ..default()
                        },
                        AttackSensor,
                        RigidBody::Dynamic,
                        Name::new("Parent Bottom Whip"),
                    ))
                    .with_children(|parent| {
                        // Thin bottom Whip
                        parent.spawn((
                            // REFACTOR: Find a way to .into() a (f32, f32) tuple into a 2arguments function
                            Collider::cuboid(
                                PLAYER_ATTACK_HITBOX_BOTTOM.0,
                                PLAYER_ATTACK_HITBOX_BOTTOM.1,
                            ),
                            Transform::default(),
                            AttackHitbox(10),
                            PlayerAttack,
                            Sensor,
                            Name::new("Attack Hitbox: Sensor Bottom Whip"),
                        ));
                    });

                parent
                    .spawn((
                        SpatialBundle {
                            transform: Transform::from_translation(FRONT_WHIP_POS.into()),
                            ..default()
                        },
                        AttackSensor,
                        RigidBody::Dynamic,
                        Name::new("Parent Front Ball"),
                    ))
                    .with_children(|parent| {
                        // Front Ball
                        parent.spawn((
                            // REFACTOR: Find a way to .into() a (f32, f32) tuple into a 2arguments function
                            Collider::cuboid(
                                PLAYER_ATTACK_HITBOX_FRONT.0,
                                PLAYER_ATTACK_HITBOX_FRONT.1,
                            ),
                            Transform::default(),
                            AttackHitbox(10),
                            PlayerAttack,
                            Sensor,
                            Name::new("Attack Hitbox: Sensor Front Ball"),
                        ));
                    });
            });
    }
}
