use crate::characters::{aggression::Hp, player::Player};
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ui).add_system(update_health);
    }
}

#[derive(Component)]
struct HealthText;

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(
            TextBundle::from_section(
                "Health: 100%",
                TextStyle {
                    font: asset_server.load("fonts/dpcomic.ttf"),
                    font_size: 30.0,
                    color: Color::GRAY,
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(10.0),
                    right: Val::Px(10.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(HealthText);
}

fn update_health(
    mut text_query: Query<&mut Text, With<HealthText>>,
    hp_query: Query<&Hp, (With<Player>, Changed<Hp>)>,
) {
    for hp in hp_query.iter() {
        let mut text = text_query.single_mut();
        text.sections[0] = TextSection::new(
            format!("Health: {}%", hp.current / hp.max),
            text.sections[0].style.clone(),
        );
    }
}
