use bevy::prelude::*;

use crate::{
    characters::{
        aggression::{AttackHitbox, Hp},
        npcs::boss::Boss,
        player::{Player, PlayerAttack},
    },
    constants::ui::*,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app .add_startup_system(setup_ui)
            .add_system(triche_system)
            // -- HP --
            .add_system(update_health)
            .add_system(display_player_hp)
            .add_system(display_boss_hp)
            ;
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

    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(200.), Val::Px(65.)),
                    // center button
                    margin: UiRect::all(Val::Auto),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    position: UiRect {
                        right: Val::Percent(41.),
                        top: Val::Percent(-44.),
                        ..default()
                    },
                    ..default()
                },
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            Name::new("Triche Button"),
            // HackButton
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "LA TRICHE",
                TextStyle {
                    font: asset_server.load("fonts/dpcomic.ttf"),
                    font_size: 40.,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
}

fn update_health(
    mut text_query: Query<&mut Text, With<HealthText>>,
    hp_query: Query<&Hp, (With<Player>, Changed<Hp>)>,
) {
    for hp in hp_query.iter() {
        let mut text = text_query.single_mut();
        text.sections[0] = TextSection::new(
            format!(
                "Health: {}%",
                ((hp.current as f32) / (hp.max as f32)) * 100.
            ),
            text.sections[0].style.clone(),
        );
    }
}

fn display_player_hp(
    bleeding_player_query: Query<&Hp, (With<Player>, Or<(Added<Hp>, Changed<Hp>)>)>,
) {
    if let Ok(player_hp) = bleeding_player_query.get_single() {
        println!("player's hp: {}/{}", player_hp.current, player_hp.max);
    }
}

/// DEBUG: TEMPORARY
///
/// The Boss' hp won't be displayed.
/// The current phase will indicate, as well as the clouds ?
pub fn display_boss_hp(
    bleeding_boss_query: Query<&Hp, (With<Boss>, Or<(Added<Hp>, Changed<Hp>)>)>,
) {
    if let Ok(boss_hp) = bleeding_boss_query.get_single() {
        println!("boss's hp: {}/{}", boss_hp.current, boss_hp.max);
    }
}

// IDEA: polish - Poping stats up and hp changes
// (usefull for every combat ever)

fn triche_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,

    mut player_attack: Query<&mut AttackHitbox, With<PlayerAttack>>,

    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                info!("+100");
                // +100dmg to the player
                for mut attack_hitbox in player_attack.iter_mut() {
                    attack_hitbox.0 += 100
                }

                text.sections[0].value = String::from("LA TRONCHE");
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                text.sections[0].value = String::from("+100dmg");
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = String::from("LA TRICHE");
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
