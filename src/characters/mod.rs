pub mod aggression;
pub mod animations;
pub mod movement;
pub mod npcs;
pub mod player;

use bevy::prelude::*;

use crate::{
    characters::{
        aggression::{AggressionPlugin, DeadBody},
        movement::{dash_timer, hyper_dash_timer, player_dash},
        animations::animate_character,
        animations::jump_frame_character_state,
        npcs::NPCsPlugin,
        player::PlayerPlugin,
    },
    locations::run_if_the_player_is_not_frozen,
};


pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app 
            // REFACTOR: Remove filters in favour of a `System Order of Execution`
            // ^^^^^^^^^------- filters used to weirdly correct bugs (whihc cause new bugs...)
            .add_plugin(NPCsPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(AggressionPlugin)
            .add_system(dash_timer)
            .add_system(player_dash)
            .add_system(hyper_dash_timer)
            // -- Animation --
            .add_system(move_dead_bodies.with_run_criteria(run_if_the_player_is_not_frozen))
            // CoreStage::Last (after player_death_event and player_attack player_movement) but should be fine
            .add_system_to_stage(
                // ensure that the changes in each CharacterPhase are made
                CoreStage::PostUpdate,
                jump_frame_character_state
                    .before(animate_character)
                    
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                animate_character
            )
            ;
    }
}

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
        const SPEED: f32 = 25.;

        transform.translation.x += -dir as f32 * SPEED * time.delta_seconds();
    }
}
