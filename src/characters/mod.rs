pub mod animations;
pub mod movement;
pub mod npcs;
pub mod player;

use bevy::prelude::*;

use self::{
    animations::animate_character, animations::jump_frame_player_state, npcs::NPCsPlugin,
    player::PlayerPlugin,
};

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app .add_plugin(NPCsPlugin)
            .add_plugin(PlayerPlugin)
            // -- Animation --
            .add_system(animate_character)
            .add_system(jump_frame_player_state)
            ;
    }
}
