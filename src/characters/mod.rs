pub mod aggression;
pub mod animations;
pub mod movement;
pub mod npcs;
pub mod player;

use bevy::prelude::*;

use crate::locations::run_if_the_player_is_not_frozen;

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
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(run_if_the_player_is_not_frozen)
                    .with_system(move_dead_bodies)
            )
            .add_system_set_to_stage(
                // ensure that the changes in each CharacterPhase are made
                CoreStage::PostUpdate,
                SystemSet::new()
                    .with_system(jump_frame_player_state.before(animate_character))
            )
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new()
                    .with_system(animate_character)
            )
            ;
    }
}

#[derive(Component)]
pub struct Invulnerable;

#[derive(Component)]
pub struct DeadBody;

#[derive(Component)]
pub struct Freeze;

/// REFACTOR: Parallax movement on static entities
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
