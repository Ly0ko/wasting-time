use bevy::prelude::*;

use crate::loading::{FontAssets, TextureAssets};
use crate::GameState;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(setup_menu))
            .add_system_set(
                SystemSet::on_update(GameState::GameOver).with_system(click_play_button),
            );
    }
}

fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    texture_assets: Res<TextureAssets>,
) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(TextBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(130.0),
                left: Val::Px(120.0),
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text {
            sections: vec![TextSection {
                value: "Your clock has run out.".to_string(),
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
        .spawn_bundle(ButtonBundle {
            image: texture_assets.button_texture.clone().into(),
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(300.0),
                    left: Val::Px(350.0),
                    ..Default::default()
                },
                size: Size::new(Val::Px(100.0), Val::Px(60.0)),
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
                        value: "Retry".to_string(),
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
