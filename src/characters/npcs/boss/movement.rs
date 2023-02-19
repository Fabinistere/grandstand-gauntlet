use bevy::prelude::*;
// use bevy_rapier2d::prelude::*;

use crate::{characters::player::Player, crowd::CrowdMember};

use super::Boss;

pub fn stare_player(
    mut boss_query: Query<(&mut TextureAtlasSprite, &Transform), With<Boss>>,
    player_query: Query<&Transform, (With<Player>, Without<CrowdMember>)>,
) {
    let (mut boss_sprite, boss_transform) = boss_query.single_mut();
    let player_transform = player_query.single();

    boss_sprite.flip_x = boss_transform.translation.x > player_transform.translation.x;
}
