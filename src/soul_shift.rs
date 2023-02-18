use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use bevy_rapier2d::prelude::*;

use crate::{
    characters::{
        animations::CharacterState,
        player::{CreatePlayerEvent, Player},
        DeadBody,
    },
    constants::character::CHAR_POSITION,
    crowd::CrowdMember,
};

pub struct SoulShiftPlugin;

impl Plugin for SoulShiftPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(start_soul_shift);
    }
}

fn start_soul_shift(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut crowd_member_query: Query<(Entity, &mut Transform), (With<CrowdMember>, Without<Player>)>,
    mut player_transform_query: Query<
        (Entity, &mut Transform, &mut CharacterState, &mut Velocity),
        With<Player>,
    >,
    mut create_player_event: EventWriter<CreatePlayerEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::E) {
        let (player_entity, mut player_transform, mut player_state, mut player_velocity) =
            player_transform_query.single_mut();

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
            None => return,
        };

        // Update current player
        *player_state = CharacterState::Dead;
        commands.entity(player_entity).remove::<Player>();
        commands.entity(player_entity).insert(DeadBody);

        // Update new player

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

        commands.entity(closest_member).remove::<CrowdMember>();
        commands.entity(closest_member).insert(Player);
        create_player_event.send(CreatePlayerEvent(closest_member));
    }
}
