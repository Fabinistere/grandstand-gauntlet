use crate::{
    characters::animations::{AnimationIndices, AnimationTimer, CharacterState},
    constants::{
        character::player::*,
        crowd::{CROWD_SIZE, CROWD_SPAN, CROWD_Y, CROWD_Z},
    },
    locations::run_if_the_player_is_not_frozen,
};
use bevy::{ecs::schedule::ShouldRun, prelude::*, utils::HashMap};
use rand::Rng;

pub struct CrowdPlugin;

impl Plugin for CrowdPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(generate_crowd.with_run_criteria(texture_not_loaded))
            .add_system(
                move_crowd_with_background.with_run_criteria(run_if_the_player_is_not_frozen),
            )
            .insert_resource(CharacterSpriteSheetLoaded(false));
    }
}

#[derive(Debug, Deref, DerefMut, Resource)]
struct CharacterSpritesheetImage(Handle<Image>);

#[derive(Debug, Deref, DerefMut, Resource)]
struct CharacterSpriteSheetLoaded(bool);

#[derive(Debug, Component)]
pub struct CrowdMember;

fn texture_not_loaded(character_spritehseet_loaded: Res<CharacterSpriteSheetLoaded>) -> ShouldRun {
    if **character_spritehseet_loaded {
        ShouldRun::No
    } else {
        ShouldRun::Yes
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image = asset_server.load("textures/character/character_spritesheet_v2.png");
    commands.insert_resource(CharacterSpritesheetImage(image));
}

fn move_crowd_with_background(
    mut query: Query<&mut Transform, With<CrowdMember>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for mut transform in query.iter_mut() {
        let right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);
        let left = keyboard_input.pressed(KeyCode::Q)
            || keyboard_input.pressed(KeyCode::A)
            || keyboard_input.pressed(KeyCode::Left);

        let dir = right as i8 - left as i8;
        const SPEED: f32 = 18.0;

        transform.translation.x += -dir as f32 * SPEED * time.delta_seconds();
    }
}

fn generate_crowd(
    mut character_spritehseet_loaded: ResMut<CharacterSpriteSheetLoaded>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut assets: ResMut<Assets<Image>>,
    character_spritesheet_image: Res<CharacterSpritesheetImage>,
) {
    if let Some(image_handle) = assets.get(&**character_spritesheet_image) {
        let mut animation_indices = AnimationIndices(HashMap::new());
        animation_indices.insert(CharacterState::Idle, PLAYER_IDLE_FRAMES);
        animation_indices.insert(CharacterState::Run, PLAYER_RUN_FRAMES);
        animation_indices.insert(
            CharacterState::TransitionToCharge,
            PLAYER_TRANSITION_TO_CHARGE_FRAMES,
        );
        animation_indices.insert(CharacterState::Charge, PLAYER_CHARGE_FRAMES);
        animation_indices.insert(CharacterState::Attack, PLAYER_CHARGED_ATTACK_FRAMES);
        animation_indices.insert(CharacterState::SecondAttack, PLAYER_SECOND_ATTACK_FRAMES);
        animation_indices.insert(CharacterState::Hit, PLAYER_HIT_FRAMES);
        animation_indices.insert(CharacterState::Dead, PLAYER_DEAD_FRAMES);

        **character_spritehseet_loaded = true;
        let image_handle = image_handle.clone();

        let mut rand = rand::thread_rng();
        let crowd_member_spacing = CROWD_SPAN * 2.0 / CROWD_SIZE as f32;
        let mut current_crowd_member_x = -CROWD_SPAN;

        let parent = commands
            .spawn((
                Name::new("Crowd"),
                GlobalTransform::default(),
                Transform::default(),
                ComputedVisibility::default(),
                Visibility::default(),
            ))
            .id();

        for count in 0..CROWD_SIZE {
            let image = image_handle.clone();
            let mut image_dynamic = image.try_into_dynamic().unwrap();
            image_dynamic = image_dynamic.huerotate(rand.gen_range(0..360));
            // .brighten(-50);

            let texture_handle = assets.add(Image::from_dynamic(image_dynamic, true));
            let texture_atlas =
                TextureAtlas::from_grid(texture_handle, Vec2::new(200., 200.), 35, 1, None, None);
            let texture_atlas_handle = texture_atlases.add(texture_atlas);

            let texture_atlas_sprite = TextureAtlasSprite::new(0);

            commands.entity(parent).with_children(|parent| {
                parent.spawn((
                    SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle,
                        sprite: texture_atlas_sprite,
                        transform: Transform::from_translation(Vec3::new(
                            current_crowd_member_x + rand.gen_range(-10.0..=10.0),
                            CROWD_Y,
                            CROWD_Z,
                        )),
                        ..default()
                    },
                    CrowdMember,
                    CharacterState::Idle,
                    AnimationTimer(Timer::from_seconds(
                        0.1 + rand.gen_range(-0.02..0.02),
                        TimerMode::Repeating,
                    )),
                    animation_indices.clone(),
                    // To match the animate_characters query (DEBUG update)
                    Name::new(format!("Spectator n°{}", count)),
                ));
            });

            current_crowd_member_x += crowd_member_spacing;
        }
    }
}
