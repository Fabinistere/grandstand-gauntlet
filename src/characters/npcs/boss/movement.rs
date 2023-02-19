use bevy::prelude::*;
// use bevy_rapier2d::prelude::*;

use crate::{
    characters::{aggression::FlipAttackSensorEvent, player::Player},
    crowd::CrowdMember,
};

use super::Boss;

pub fn stare_player(
    mut boss_query: Query<(Entity, &mut TextureAtlasSprite, &Transform), With<Boss>>,
    player_query: Query<&Transform, (With<Player>, Without<CrowdMember>)>,
    mut flip_direction_event: EventWriter<FlipAttackSensorEvent>,
) {
    let (boss, mut boss_sprite, boss_transform) = boss_query.single_mut();
    let player_transform = player_query.single();

    if boss_sprite.flip_x != (boss_transform.translation.x > player_transform.translation.x) {
        flip_direction_event.send(FlipAttackSensorEvent(boss));
    }
    boss_sprite.flip_x = boss_transform.translation.x > player_transform.translation.x;
}
