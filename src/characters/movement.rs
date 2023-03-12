use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::Velocity;
// use bevy_retrograde::prelude::Velocity;

use crate::TILE_SIZE;

use super::{npcs::boss::Boss, player::Player};

// find the right place to put this component (indicator)
#[derive(Component)]
pub struct CharacterHitbox;

#[derive(Component, Deref, DerefMut, Inspectable)]
pub struct Speed(pub f32);

impl Default for Speed {
    fn default() -> Self {
        Speed(50. * TILE_SIZE)
    }
}

impl Speed {
    pub fn new(speed: f32) -> Self {
        Speed(speed * TILE_SIZE)
    }
}

#[derive(Bundle)]
pub struct MovementBundle {
    pub speed: Speed,
    pub velocity: Velocity,
}

#[derive(Component, Debug, Deref, DerefMut)]
pub struct DashTimer(pub Timer);

#[derive(Component, Debug, Deref, DerefMut)]
pub struct HyperDashTimer(pub Timer);

pub fn dash_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut character_query: Query<
        (Entity, &mut DashTimer, &mut Velocity, &TextureAtlasSprite),
        Or<(With<Player>, With<Boss>)>,
    >,
) {
    if let Ok((character, mut dash, mut character_vel, texture_atlas_sprite)) =
        character_query.get_single_mut()
    {
        dash.tick(time.delta());

        if dash.just_finished() {
            character_vel.linvel = Vec2::ZERO;
            commands.entity(character).remove::<DashTimer>();
            // dashing while in invul will replace the current invul
            // .remove::<Invulnerable>();
        } else {
            // TODO: put this as const `200. * time.delta_seconds()`

            // Ultra dash = +-500. * 200. * time.delta_seconds()
            character_vel.linvel.x = if texture_atlas_sprite.flip_x {
                -100. * 200. * time.delta_seconds()
            } else {
                100. * 200. * time.delta_seconds()
            };
        }
    }
}

/// Reserved for the player
pub fn hyper_dash_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut player_query: Query<
        (
            Entity,
            &mut HyperDashTimer,
            &mut Velocity,
            &TextureAtlasSprite,
        ),
        With<Player>,
    >,
) {
    if let Ok((player, mut dash, mut player_vel, texture_atlas_sprite)) =
        player_query.get_single_mut()
    {
        dash.tick(time.delta());

        if dash.just_finished() {
            player_vel.linvel = Vec2::ZERO;
            commands.entity(player).remove::<HyperDashTimer>();
            // dashing while in invul will replace the current invul
            // .remove::<Invulnerable>();
        } else {
            // TODO: put this as const `200. * time.delta_seconds()`
            player_vel.linvel.x = if texture_atlas_sprite.flip_x {
                -500. * 200. * time.delta_seconds()
            } else {
                500. * 200. * time.delta_seconds()
            };
        }
    }
}
