use bevy::{prelude::*, utils::HashMap};

use super::{npcs::boss::Boss, player::Player};
use crate::crowd::CrowdMember;

#[derive(Component, PartialEq, Eq, Hash, Clone)]
pub enum CharacterState {
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
            let indices = indices[&character_state];
            // REFACTOR: the limit being modified by magical number
            let limit: usize;
            let start_back: usize;
            let state_when_restart: Option<CharacterState>;

            if *character_state == CharacterState::Attack
                || *character_state == CharacterState::SecondAttack
            {
                // Idle
                start_back = 0;
                state_when_restart = Some(CharacterState::Idle);
                // End of SecondAttack
                limit = 26;
            } else if *character_state == CharacterState::Run {
                // Idle
                start_back = 0;
                state_when_restart = Some(CharacterState::Idle);
                // End of SecondAttack
                limit = 12;
            } else {
                // Loop
                start_back = indices.0;
                state_when_restart = None;
                limit = indices.1;
            }

            if sprite.index == limit {
                sprite.index = start_back;
                // update state
                match state_when_restart {
                    Some(new_state) => *character_state = new_state,
                    None => continue,
                }
            } else {
                sprite.index = sprite.index + 1
            };
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
