use bevy::{prelude::*, utils::HashMap};
use bevy_inspector_egui::Inspectable;

use crate::{
    characters::{aggression::DeadBody, npcs::boss::Boss, player::Player},
    crowd::CrowdMember,
};

use super::npcs::boss::behaviors::{ActionCompletedEvent, BossAction, BossActions};

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
    Dash,
    Hit,
    Dead,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

/// A CharacterState is linked to
///
/// - a start_index (first frame),
/// - a end_index (last frame),
/// - the next CharacterState (after the anim ended)
#[derive(Component, Deref, DerefMut, Clone)]
pub struct AnimationIndices(pub HashMap<CharacterState, (usize, usize, CharacterState)>);

/// # Note
///
/// TODO: longer animation of "getting hit"
pub fn animate_character(
    mut commands: Commands,

    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<
        (
            Entity,
            &AnimationIndices,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
            &mut CharacterState,
            &Name,
        ),
        Or<(With<Player>, With<Boss>, With<CrowdMember>, With<DeadBody>)>,
    >,

    boss_actions_query: Query<&BossActions>,
    mut action_completed_event: EventWriter<ActionCompletedEvent>,
) {
    for (
        character,
        indices,
        mut timer,
        mut sprite,
        texture_atlas_handle,
        mut character_state,
        name,
    ) in &mut query
    {
        timer.tick(time.delta());

        if timer.just_finished() {
            let (_first_frame, last_frame, next_phase) = &indices[&character_state];

            // TODO: longer animation of "getting hit"
            // IDEA: Invulnerable Hint - see characters::aggrssion::invulnerability_timer

            // eprintln!("{:#?}", sprite);

            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();

            if sprite.index == *last_frame {
                // Final Frame of Death
                if *character_state == CharacterState::Dead {
                    commands.entity(character).remove::<AnimationTimer>();
                } else {
                    // --- Boss AI ---
                    if let Ok(boss_actions) = boss_actions_query.get(character) {
                        match &boss_actions.0 {
                            None => continue,
                            Some(actions) => {
                                // shouldn't crash
                                match actions[0] {
                                    BossAction::Smash => {
                                        if *character_state == CharacterState::Attack {
                                            action_completed_event.send(ActionCompletedEvent);
                                        }
                                    }
                                    BossAction::FallenAngel => {
                                        if *character_state == CharacterState::SecondAttack {
                                            action_completed_event.send(ActionCompletedEvent);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }

                    // starting on the start frame of the 'new' phase
                    sprite.index = indices[next_phase].0;
                    // update state
                    *character_state = next_phase.clone();
                }
            } else if sprite.index + 1 < texture_atlas.textures.len() {
                sprite.index = sprite.index + 1
            } else {
                warn!("anim limit reached: {}", name);
                // commands.entity(character).remove::<AnimationTimer>();
                sprite.index = indices[next_phase].0;
            }
        }
    }
}

/// Anytime the CharacterState change,
/// force the sprite to match this change.
pub fn jump_frame_character_state(
    mut query: Query<
        (&AnimationIndices, &mut TextureAtlasSprite, &CharacterState),
        Changed<CharacterState>,
    >,
) {
    for (indices, mut sprite, character_state) in &mut query {
        // info!("{:?}", &character_state);
        let (first_indice, _, _) = &indices[&character_state];
        // Jump directly to the correct frame when the state has changed
        sprite.index = *first_indice;
    }
}
