pub mod aggression;
pub mod animations;
pub mod movement;
pub mod npcs;
pub mod player;

use bevy::prelude::*;

use self::{
    aggression::AggressionPlugin, animations::animate_character,
    animations::jump_frame_player_state, player::PlayerPlugin, npcs::NPCsPlugin,
};

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app 
            .add_plugin(NPCsPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(AggressionPlugin)
            // -- Animation --
            .add_system(animate_character)
            .add_system(jump_frame_player_state)
            .add_system(move_dead_bodies)
            ;
    }
}

#[derive(Component)]
pub struct Invulnerable;

#[derive(Component)]
pub struct DeadBody;

fn move_dead_bodies(
    mut query: Query<&mut Transform, With<DeadBody>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for mut transform in query.iter_mut() {
        let right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);
        let left = keyboard_input.pressed(KeyCode::Q)
            || keyboard_input.pressed(KeyCode::A)
            || keyboard_input.pressed(KeyCode::Left);

        let dir = right as i8 - left as i8;
        const SPEED: f32 = 25.0;

        transform.translation.x += -dir as f32 * SPEED * time.delta_seconds();
    }
}
