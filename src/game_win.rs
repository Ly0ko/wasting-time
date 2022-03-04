use bevy::prelude::*;

use crate::loading::{FontAssets, TextureAssets};
use crate::GameState;

struct FastTime {
    pub minutes: f32,
    pub seconds: f32,
    pub time: f32,
}

#[derive(Component)]
struct FastTimeUI;

pub struct GameWinPlugin;

impl Plugin for GameWinPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::GameWin).with_system(setup_menu))
            .add_system_set(
                SystemSet::on_update(GameState::GameWin)
                    .with_system(click_play_button)
                    .with_system(update_timer),
            );
    }
}

fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    texture_assets: Res<TextureAssets>,
) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands.insert_resource(FastTime {
        minutes: 0.0,
        seconds: 0.0,
        time: 800.0,
    });
    commands.spawn_bundle(TextBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(130.0),
                left: Val::Px(130.0),
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text {
            sections: vec![TextSection {
                value: "You have wasted time".to_string(),
                style: TextStyle {
                    font: font_assets.roboto.clone(),
                    font_size: 70.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            }],
            alignment: Default::default(),
        },
        ..Default::default()
    });
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(220.0),
                    left: Val::Px(350.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: "99:99".to_string(),
                    style: TextStyle {
                        font: font_assets.roboto.clone(),
                        font_size: 50.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                }],
                alignment: Default::default(),
            },
            ..Default::default()
        })
        .insert(FastTimeUI);
    commands
        .spawn_bundle(ButtonBundle {
            image: texture_assets.button_texture.clone().into(),
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(300.0),
                    left: Val::Px(320.0),
                    ..Default::default()
                },
                size: Size::new(Val::Px(170.0), Val::Px(60.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(12.0),
                        left: Val::Px(12.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text {
                    sections: vec![TextSection {
                        value: "Play Again".to_string(),
                        style: TextStyle {
                            font: font_assets.roboto.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            });
        });
}

fn update_timer(
    mut q_text: Query<&mut Text, With<FastTimeUI>>,
    mut timer: ResMut<FastTime>,
    time: Res<Time>,
) {
    timer.time += time.delta_seconds() * 20.0;
    timer.minutes = (timer.time / 60.0).floor();
    timer.seconds = (timer.time % 60.0).floor();

    let mut text = q_text.single_mut();
    text.sections[0].value = format!("{:02}:{:02}", timer.minutes, timer.seconds);
}

type ButtonInteraction<'a> = (Entity, &'a Interaction, &'a Children);

fn click_play_button(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    q_interaction: Query<ButtonInteraction, (Changed<Interaction>, With<Button>)>,
    q_text: Query<Entity, With<Text>>,
) {
    for (button, interaction, _) in q_interaction.iter() {
        if *interaction == Interaction::Clicked {
            commands.entity(button).despawn();
            for text in q_text.iter() {
                commands.entity(text).despawn();
            }
            state.set(GameState::Playing).unwrap();
        }
    }
}
