use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    characters::{
        aggression::Hp,
        animations::CharacterState,
        player::{CreatePlayerEvent, Player, PlayerDeathEvent},
        DeadBody,
    },
    constants::character::CHAR_POSITION,
    crowd::CrowdMember,
};

pub struct SoulShiftPlugin;

impl Plugin for SoulShiftPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SoulShiftEvent>()
            .add_system(start_soul_shift.label("Soul Shift"))
            .add_system(suicide_to_soul_shift);
    }
}

// DEBUG: Remove it maybe ?
#[derive(Component)]
pub struct SoulShifting;

/// Happens when
///   - soul_shift::suicide_to_soul_shift
///     - press e
///   - characters::aggression::damage_hit
///     - Player's hp is = 0
///
/// Read in
///   - soul_shift::start_soul_shift
///     - SOUL SHIFTING // DOC: short explanation on whats happening
pub struct SoulShiftEvent(pub Entity);

fn suicide_to_soul_shift(
    keyboard_input: Res<Input<KeyCode>>,
    mut soul_shift_event: EventWriter<SoulShiftEvent>,
    mut player_query: Query<(Entity, &mut Hp), (With<Player>, Without<DeadBody>)>,
) {
    if keyboard_input.just_pressed(KeyCode::E) {
        if let Ok((player, mut hp)) = player_query.get_single_mut() {
            // So long
            hp.current = 0;
            soul_shift_event.send(SoulShiftEvent(player));
        }
    }
}

pub fn start_soul_shift(
    mut commands: Commands,

    mut soul_shift_event: EventReader<SoulShiftEvent>,

    mut crowd_member_query: Query<(Entity, &mut Transform), (With<CrowdMember>, Without<Player>)>,
    mut player_query: Query<
        (
            Entity,
            &mut Transform,
            &mut CharacterState,
            &mut Velocity,
            &Hp,
            &Name,
        ),
        (
            With<Player>,
            Without<DeadBody>,
            Without<SoulShifting>,
            Without<CrowdMember>,
        ),
    >,
    mut death_event: EventWriter<PlayerDeathEvent>,
    mut create_player_event: EventWriter<CreatePlayerEvent>,
) {
    for SoulShiftEvent(entity) in soul_shift_event.iter() {
        match player_query.get_mut(*entity) {
            Err(e) => warn!(
                "The Entity asked for a SoulShift is already treated: {:?}",
                e
            ),
            // Should pary all the doublon event on the lethal move
            Ok((
                player_entity,
                mut player_transform,
                mut player_state,
                mut player_velocity,
                player_hp,
                player_name,
            )) => {
                if player_hp.current == 0 {
                    info!("Successfull Soul Shift Event");

                    let mut closest_member = None;
                    let mut min_distance = f32::MAX;

                    for (entity, transform) in crowd_member_query.iter() {
                        let distance = player_transform.translation.distance(transform.translation);

                        if distance < min_distance {
                            min_distance = distance;
                            closest_member = Some(entity);
                        }
                    }

                    let closest_member = match closest_member {
                        Some(e) => e,
                        // TODO: End of the Game (no more life left) Sadge !
                        None => return,
                    };

                    // ------- Kill for good the old body -------

                    *player_state = CharacterState::Dead;
                    commands
                        .entity(player_entity)
                        .insert((SoulShifting, DeadBody))
                        .remove::<Player>();
                    // DEBUG: maybe will be too slow and all single on Player will break
                    // ^^^^^^------ System Ordering
                    death_event.send(PlayerDeathEvent(player_entity));

                    // ------- Update new player -------

                    {
                        let mut transform = crowd_member_query
                            .get_component_mut::<Transform>(closest_member)
                            .unwrap();
                        transform.translation.y = CHAR_POSITION.1;
                        transform.translation.z = CHAR_POSITION.2;
                    }

                    player_velocity.linvel = Vect::ZERO;
                    // player_transform.translation.z = CROWD_Z;
                    player_transform.translation.y = CHAR_POSITION.1 - 5.0;

                    commands
                        .entity(closest_member)
                        .insert((Player, SoulShifting))
                        .remove::<CrowdMember>();
                    create_player_event.send(CreatePlayerEvent(closest_member));
                } else {
                    warn!(
                        "This entity {}:{:?} is not yet dying",
                        player_name, player_entity
                    )
                }
            }
        }
    }
}
