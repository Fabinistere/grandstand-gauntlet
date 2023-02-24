use bevy::prelude::*;

use crate::{characters::player::Player, crowd::CrowdMember};

/// The camera follows the current controled entity
///
/// # Note
///
/// IDEA: gamefeel - smooth transition between mind control switch
pub fn camera_follow(
    player_query: Query<&Transform, (With<Player>, Without<CrowdMember>)>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = camera_transform
        .translation
        .lerp(player_transform.translation, 0.5)
        .x;
}
