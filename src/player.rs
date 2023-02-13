use bevy::{prelude::*, utils::HashMap};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_player)
            .add_system(animate_player);
    }
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct AnimationIndices(HashMap<PlayerState, (usize, usize)>);

#[derive(Component)]
pub struct Player;

#[derive(Component, PartialEq, Eq, Hash)]
enum PlayerState {
    Idle,
    Attack,
    SecondAttack,
    TransitionToCharge,
    Charge,
    Run,
    Hit,
    Dead,
}

fn animate_player(
    time: Res<Time>,
    mut query: Query<
        (
            &AnimationIndices,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &PlayerState,
        ),
        With<Player>,
    >,
) {
    for (indices, mut timer, mut sprite, player_state) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            let indices = indices[player_state];

            sprite.index = if sprite.index == indices.1 {
                indices.0
            } else {
                sprite.index + 1
            };
        }
    }
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut animation_indices = AnimationIndices(HashMap::new());
    animation_indices.insert(PlayerState::Idle, (0, 4));
    animation_indices.insert(PlayerState::Attack, (19, 23));
    animation_indices.insert(PlayerState::SecondAttack, (24, 26));
    animation_indices.insert(PlayerState::TransitionToCharge, (13, 14));
    animation_indices.insert(PlayerState::Charge, (15, 18));
    animation_indices.insert(PlayerState::Run, (5, 12));
    animation_indices.insert(PlayerState::Hit, (27, 28));
    animation_indices.insert(PlayerState::Dead, (29, 33));

    let texture_handle = asset_server.load("textures/character/character_spritesheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(122.0, 122.0), 34, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let texture_atlas_sprite = TextureAtlasSprite::new(0);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: texture_atlas_sprite,
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        animation_indices,
        Player,
        PlayerState::Idle,
    ));
}
