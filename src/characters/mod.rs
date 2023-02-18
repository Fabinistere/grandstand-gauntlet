mod aggression;
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
            ;
    }
}

#[derive(Component)]
pub struct Invulnerable;
