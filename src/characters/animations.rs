use bevy::{prelude::*, utils::HashMap};
use bevy_inspector_egui::Inspectable;

use crate::{
    characters::{aggression::DeadBody, npcs::boss::Boss, player::Player},
    crowd::CrowdMember,
};

#[derive(Default, Debug, Clone, Component, Eq, Hash, Inspectable, PartialEq)]
pub enum CharacterState {
    #[default]
    Idle,
    Attack,
    SecondAttack,
    ChargedAttack,
    TransitionToCharge,
    Charge,
    Run,
    Hit,
    Dead,
    // OPTIMIZE: Stop animate
    // PermaDeath,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Deref, DerefMut, Clone)]
pub struct AnimationIndices(pub HashMap<CharacterState, (usize, usize)>);

/// # Note
///
/// TODO: longer animation of "getting hit"
pub fn animate_character(
    mut commands: Commands,

    time: Res<Time>,
    mut query: Query<
        (
            Entity,
            &AnimationIndices,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &mut CharacterState,
        ),
        Or<(With<Player>, With<Boss>, With<CrowdMember>, With<DeadBody>)>,
    >,
) {
    for (character, indices, mut timer, mut sprite, mut character_state) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            let current_indices = indices[&character_state];
            let next_phase: Option<CharacterState>;

            if *character_state == CharacterState::Run
                || *character_state == CharacterState::Attack
                || *character_state == CharacterState::SecondAttack
                || *character_state == CharacterState::ChargedAttack
                || *character_state == CharacterState::Hit
            {
                // TODO: longer animation of "getting hit"
                // Idle when stop running/attacking/getting hit
                next_phase = Some(CharacterState::Idle);
            } else if *character_state == CharacterState::TransitionToCharge {
                // Charging
                next_phase = Some(CharacterState::Charge);
            } else if *character_state == CharacterState::Dead {
                // CharacterState::PermaDeath (last frame to last frame)
                next_phase = Some(CharacterState::Dead);
            } else {
                // Loop
                next_phase = None;
            }

            if sprite.index == current_indices.1 {
                // Final Frame of Death
                if *character_state == CharacterState::Dead {
                    commands.entity(character).remove::<AnimationTimer>();
                } else {
                    match next_phase {
                        Some(new_state) => {
                            sprite.index = indices[&new_state].0;
                            // update state
                            *character_state = new_state;
                        }
                        None => {
                            sprite.index = current_indices.0;
                        }
                    }
                }
            } else {
                sprite.index = sprite.index + 1
            }
        }
    }
}

/// Anytime the CharacterState change,
/// force the sprite to match this change.
pub fn jump_frame_player_state(
    mut query: Query<
        (&AnimationIndices, &mut TextureAtlasSprite, &CharacterState),
        (Or<(With<Player>, With<Boss>)>, Changed<CharacterState>),
    >,
) {
    for (indices, mut sprite, player_state) in &mut query {
        let indices = indices[&player_state];
        // Jump directly to the correct frame when the state has changed
        sprite.index = indices.0;
    }
}
