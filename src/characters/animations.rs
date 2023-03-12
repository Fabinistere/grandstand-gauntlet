use bevy::{prelude::*, utils::HashMap};
use bevy_inspector_egui::Inspectable;

use crate::{
    characters::{aggression::DeadBody, npcs::boss::Boss, player::Player},
    crowd::CrowdMember,
};

use super::{
    movement::DashTimer,
    npcs::boss::behaviors::{ActionCompletedEvent, BossAction, BossActions},
};

#[derive(Default, Debug, Clone, Component, Eq, Hash, Inspectable, PartialEq)]
pub enum CharacterState {
    #[default]
    Idle,
    Feint,
    Attack,
    SecondAttack,
    ThirdAttack,
    ChargedAttack,
    TransitionToCharge,
    Charge,
    Run,
    TransitionToDash,
    Dash,
    TpOut,
    TpIn,
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

/// Happens when
///   - animation::animate_character
///     - A boss get to the last frame of one of their AnimState
///     
/// Read in
///   - animation:boss_last_frame
///     - Send a Action Completed Event whether
///     the state is consistent with the current action
pub struct BossLastFrameEvent(pub CharacterState);

/// Animates all but Bosses
///
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

    boss_query: Query<Entity, With<Boss>>,
    mut boss_last_frame_event: EventWriter<BossLastFrameEvent>,
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
                    match boss_query.get(character) {
                        Err(_) => {}
                        Ok(_) => {
                            boss_last_frame_event.send(BossLastFrameEvent(character_state.clone()));
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

/// Animates only Bosses
///
/// # Note
///
/// TODO: longer animation of "getting hit"
pub fn boss_last_frame(
    mut boss_last_frame_event: EventReader<BossLastFrameEvent>,

    mut commands: Commands,
    mut query: Query<(Entity, &BossActions), With<Boss>>,
    mut action_completed_event: EventWriter<ActionCompletedEvent>,
) {
    for BossLastFrameEvent(boss_state) in boss_last_frame_event.iter() {
        for (boss, boss_actions) in &mut query {
            // --- Boss AI ---
            match &boss_actions.0 {
                None => continue,
                Some(actions) => {
                    // shouldn't crash
                    match actions[0] {
                        BossAction::Smash => {
                            if *boss_state == CharacterState::Attack {
                                action_completed_event.send(ActionCompletedEvent);
                            }
                        }
                        BossAction::FallenAngel => {
                            if *boss_state == CharacterState::SecondAttack {
                                action_completed_event.send(ActionCompletedEvent);
                            }
                        }
                        BossAction::Dash => {
                            if *boss_state == CharacterState::Dash {
                                action_completed_event.send(ActionCompletedEvent);
                            } else if *boss_state == CharacterState::TransitionToDash {
                                commands.entity(boss).insert((
                                    DashTimer(Timer::from_seconds(0.2, TimerMode::Once)),
                                    // Invulnerable(Timer::from_seconds(0.2, TimerMode::Once)),
                                ));
                            }
                        }
                        _ => {}
                    }
                }
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
