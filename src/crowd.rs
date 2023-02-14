use crate::constants::CROWD_CHARACTER_Z;
use bevy::{
    ecs::schedule::ShouldRun,
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};
use rand::Rng;

pub struct CrowdPlugin;

impl Plugin for CrowdPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, setup)
            .add_system(generate_crowd.with_run_criteria(texture_not_loaded))
            .add_plugin(Material2dPlugin::<CrowdMaterial>::default())
            .insert_resource(CharacterSpriteSheetLoaded(false));
    }
}

#[derive(Debug, Deref, DerefMut, Resource)]
struct CharacterSpritesheetImage(Handle<Image>);

#[derive(Debug, Deref, DerefMut, Resource)]
struct CharacterSpriteSheetLoaded(bool);

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "74cfb505-0694-4ba0-bc81-3532cb0e69ce"]
pub struct CrowdMaterial {
    #[uniform(0)]
    color: Color,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Handle<Image>,
}

impl Material2d for CrowdMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/crowd_material.wgsl".into()
    }
}

fn texture_not_loaded(character_spritehseet_loaded: Res<CharacterSpriteSheetLoaded>) -> ShouldRun {
    if **character_spritehseet_loaded {
        ShouldRun::No
    } else {
        ShouldRun::Yes
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<CrowdMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(MaterialMesh2dBundle {
        material: materials.add(CrowdMaterial {
            color: Color::RED,
            color_texture: asset_server.load("textures/character/character_spritesheet.png"),
        }),
        ..default()
    });

    let image = asset_server.load("textures/character/character_spritesheet.png");
    commands.insert_resource(CharacterSpritesheetImage(image));
}

fn generate_crowd(
    mut character_spritehseet_loaded: ResMut<CharacterSpriteSheetLoaded>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut assets: ResMut<Assets<Image>>,
    character_spritesheet_image: Res<CharacterSpritesheetImage>,
) {
    if let Some(image_handle) = assets.get(&**character_spritesheet_image) {
        **character_spritehseet_loaded = true;
        let image_handle = image_handle.clone();

        let mut transform = 10.0;

        for _ in 0..5 {
            let image = image_handle.clone();
            let mut image_dynamic = image.try_into_dynamic().unwrap();

            for image::Rgba([r, g, b, a]) in image_dynamic.to_rgba8().pixels_mut() {
                if *a > 0 {
                    *r += ((255 - *r) as f32 * 1.0) as u8;
                    *g += ((255 - *g) as f32 * 1.0) as u8;
                    *b += ((255 - *b) as f32 * 0.0) as u8;
                }
            }

            let mut rand = rand::thread_rng();
            image_dynamic = image_dynamic.huerotate(rand.gen_range(0..360));
            // .brighten(-50);

            let texture_handle = assets.add(Image::from_dynamic(image_dynamic, true));
            let texture_atlas =
                TextureAtlas::from_grid(texture_handle, Vec2::new(122.0, 122.0), 34, 1, None, None);
            let texture_atlas_handle = texture_atlases.add(texture_atlas);

            let texture_atlas_sprite = TextureAtlasSprite::new(0);

            commands.spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: texture_atlas_sprite,
                transform: Transform::from_translation(Vec3::new(
                    transform,
                    -55.0,
                    CROWD_CHARACTER_Z,
                )),
                ..default()
            });

            transform += 30.0;
        }
    }
}
