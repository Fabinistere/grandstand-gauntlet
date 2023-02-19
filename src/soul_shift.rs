use bevy::prelude::*;
// use bevy_rapier2d::prelude::Velocity;
// use bevy_rapier2d::prelude::*;

use crate::{
    characters::player::{CreatePlayerEvent, Player, PlayerDeathEvent},
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
pub struct SoulShiftEvent;

fn suicide_to_soul_shift(
    keyboard_input: Res<Input<KeyCode>>,
    mut soul_shift_event: EventWriter<SoulShiftEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::E) {
        soul_shift_event.send(SoulShiftEvent);
    }
}

pub fn start_soul_shift(
    mut commands: Commands,

    mut soul_shift_event: EventReader<SoulShiftEvent>,

    crowd_member_query: Query<(Entity, &Transform), With<CrowdMember>>,
    player_transform_query: Query<(Entity, &Transform), With<Player>>,

    mut death_event: EventWriter<PlayerDeathEvent>,
    mut create_player_event: EventWriter<CreatePlayerEvent>,
) {
    for _ in soul_shift_event.iter() {
        info!("Soul Shift Event");
        let (player_entity, player_transform) = player_transform_query.single();

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

        commands
            .entity(player_entity)
            .insert(SoulShifting)
            .remove::<Player>();
        // DEBUG: maybe will be too slow and all single on Player will break
        // ^^^^^^------ System Ordering
        death_event.send(PlayerDeathEvent(player_entity));

        // ------- Update new player -------
        commands
            .entity(closest_member)
            .insert((Player, SoulShifting))
            .remove::<CrowdMember>();
        create_player_event.send(CreatePlayerEvent(closest_member));
    }
}
