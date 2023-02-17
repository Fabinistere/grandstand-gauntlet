use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use bevy_rapier2d::prelude::*;

use crate::{
    characters::{
        animations::CharacterState,
        movement::{MovementBundle, Speed},
        player::{CreatePlayerEvent, Player},
    },
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
    crowd_member_query: Query<(Entity, &Transform), With<CrowdMember>>,
    mut player_transform_query: Query<(Entity, &Transform, &mut CharacterState), With<Player>>,
    mut create_player_event: EventWriter<CreatePlayerEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::E) {
        let (player_entity, player_transform, mut player_state) =
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

        // Update new player
        commands.entity(closest_member).remove::<CrowdMember>();
        commands.entity(closest_member).insert(Player);
        create_player_event.send(CreatePlayerEvent(closest_member));
    }
}
