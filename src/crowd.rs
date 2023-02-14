use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

pub struct CrowdPlugin;

impl Plugin for CrowdPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, setup)
            .add_event::<GenerateCrowdEvent>()
            .add_system(generate_random_color_character)
            .add_plugin(Material2dPlugin::<CrowdMaterial>::default());
    }
}

#[derive(Debug, Deref, DerefMut, Resource)]
pub struct CharacterSpritesheetImage(Handle<Image>);

pub struct GenerateCrowdEvent;

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

fn setup(
    mut generate_crowd_event: EventWriter<GenerateCrowdEvent>,
    mut commands: Commands,
    mut materials: ResMut<Assets<CrowdMaterial>>,
    asset_server: Res<AssetServer>,
    mut assets: ResMut<Assets<Image>>,
) {
    commands.spawn(MaterialMesh2dBundle {
        material: materials.add(CrowdMaterial {
            color: Color::RED,
            color_texture: asset_server.load("textures/character/character_spritesheet.png"),
        }),
        ..default()
    });

    let mut image: Handle<Image> =
        asset_server.load("textures/character/character_spritesheet.png");
    image.make_strong(&assets);
    commands.insert_resource(CharacterSpritesheetImage(image));

    generate_crowd_event.send(GenerateCrowdEvent);
}

fn generate_random_color_character(
    mut generate_crowd_event: EventReader<GenerateCrowdEvent>,
    asset_server: Res<AssetServer>,
    mut assets: ResMut<Assets<Image>>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    character_spritesheet_image: Res<CharacterSpritesheetImage>,
) {
    for GenerateCrowdEvent in generate_crowd_event.iter() {
        let image = assets.get(&character_spritesheet_image).unwrap().clone();
        let image_dynamic = image.try_into_dynamic().unwrap();

        for image::Rgba([r, g, b, a]) in image_dynamic.to_rgba16().pixels_mut() {
            if *a > 0 {
                *r += (255 - 200) / 4;
                *g += (255 - 200) / 4;
                *b += (255 - 1) / 4;
            }
        }

        let texture_handle = assets.add(Image::from_dynamic(image_dynamic, false));
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(122.0, 122.0), 34, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let texture_atlas_sprite = TextureAtlasSprite::new(0);

        commands.spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: texture_atlas_sprite,
            transform: Transform::from_translation(Vec3::new(5.0, 0.0, 5.0)),
            ..default()
        });
    }
}
