use bevy::{prelude::*, utils::HashMap};

use super::{npcs::boss::Boss, player::Player};
use crate::crowd::CrowdMember;

#[derive(Default, Clone, Component, Eq, Hash, PartialEq)]
pub enum CharacterState {
    #[default]
    Idle,
    Attack,
    SecondAttack,
    TransitionToCharge,
    Charge,
    Run,
    Hit,
    Dead,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Deref, DerefMut, Clone)]
pub struct AnimationIndices(pub HashMap<CharacterState, (usize, usize)>);

/// # Note
///
/// REFACTOR: Crappy solution right there
pub fn animate_character(
    time: Res<Time>,
    mut query: Query<
        (
            &AnimationIndices,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &mut CharacterState,
        ),
        Or<(With<Player>, With<Boss>, With<CrowdMember>)>,
    >,
) {
    for (indices, mut timer, mut sprite, mut character_state) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            let current_indices = indices[&character_state];
            let next_phase: Option<CharacterState>;

            if *character_state == CharacterState::Run
                || *character_state == CharacterState::Attack
                || *character_state == CharacterState::SecondAttack
                || *character_state == CharacterState::TransitionToCharge
            {
                // Idle when stop running/attacking
                next_phase = Some(CharacterState::Idle);
            } else {
                // Loop
                next_phase = None;
            }

            if sprite.index == current_indices.1 {
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
        // Jump directly to the correct frame
        sprite.index = indices.0;
    }
}
