use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    characters::{
        aggression::FlipAttackSensorEvent,
        animations::CharacterState,
        movement::Speed,
        npcs::boss::{behaviors::BossBehavior, Boss},
        player::Player,
        Freeze,
    },
    crowd::CrowdMember,
};

pub fn stare_player(
    mut boss_query: Query<
        (Entity, &mut TextureAtlasSprite, &Transform, &CharacterState),
        With<Boss>,
    >,
    player_query: Query<&Transform, (With<Player>, Without<CrowdMember>)>,
    mut flip_direction_event: EventWriter<FlipAttackSensorEvent>,
) {
    let (boss, mut boss_sprite, boss_transform, boss_state) = boss_query.single_mut();
    let player_transform = player_query.single();

    // MUST-HAVE - Disable turn/movement when the boss attack (avoid spinning attack when passing behind the boss)
    // ^^^^^------ With Dash/Death TP for example

    // If boss is attacking, don't allow them to flip
    if *boss_state == CharacterState::TransitionToCharge
        || *boss_state == CharacterState::Attack
        // || *boss_state == CharacterState::Charge
        || *boss_state == CharacterState::SecondAttack
        || *boss_state == CharacterState::ChargedAttack
    {
        return;
    }

    let current_flip = boss_transform.translation.x > player_transform.translation.x;

    // Flip their sensors
    if boss_sprite.flip_x != current_flip {
        flip_direction_event.send(FlipAttackSensorEvent(boss));
    }
    // Flip their sprite
    boss_sprite.flip_x = current_flip;
}

/// # Note
///
/// TODO: An attack/stroke must be prioritized before the anim run/idle.
pub fn chase_player(
    mut commands: Commands,

    mut boss_query: Query<
        (
            Entity,
            &mut CharacterState,
            &Transform,
            &Speed,
            &mut Velocity,
            &BossBehavior,
        ),
        With<Boss>,
    >,
    player_query: Query<&Transform, (With<Player>, Without<CrowdMember>)>,
    time: Res<Time>,
) {
    if let Ok((boss, mut boss_state, boss_transform, speed, mut boss_vel, behavior)) =
        boss_query.get_single_mut()
    {
        // REFACTOR / OPTIMIZE: A component for this particular Chase Behavior
        if *behavior != BossBehavior::Chase {
            return;
        }
        // If boss is attacking, don't allow them to move
        if *boss_state == CharacterState::TransitionToCharge
            || *boss_state == CharacterState::Attack
            // || *boss_state == CharacterState::Charge
            || *boss_state == CharacterState::SecondAttack
            || *boss_state == CharacterState::ChargedAttack
        {
            boss_vel.linvel = Vect::ZERO;
            commands.entity(boss).insert(Freeze);
            return;
        }
        commands.entity(boss).remove::<Freeze>();

        let player_transform = player_query.single();

        let direction = player_transform.translation;

        let left = direction.x < boss_transform.translation.x;
        let right = direction.x > boss_transform.translation.x;

        let close_range_width = boss_transform.scale.x * 40.;

        // The boss is in range with the player
        if direction.x - close_range_width < boss_transform.translation.x
            && direction.x + close_range_width > boss_transform.translation.x
        {
            boss_vel.linvel = Vect::ZERO;
            // TODO: New beahvior if in range
        } else {
            // The boss is away from the player

            let x_axis = -(left as i8) + right as i8;

            boss_vel.linvel.x = x_axis as f32 * **speed * 200. * time.delta_seconds();
        }

        // ---- Animation ----

        // if there is any movement
        if (left || right) && *boss_state != CharacterState::Run {
            *boss_state = CharacterState::Run;
        } else if !(left || right) && *boss_state == CharacterState::Run {
            *boss_state = CharacterState::Idle;
        }
    }
}
