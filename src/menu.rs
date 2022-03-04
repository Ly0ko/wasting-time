use bevy::prelude::*;

use crate::loading::{FontAssets, TextureAssets};
use crate::GameState;

pub struct MenuPlugin;

#[derive(Component)]
struct TitleImage;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup_menu))
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(click_play_button));
    }
}

fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    texture_assets: Res<TextureAssets>,
) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(30.0),
                    left: Val::Px(100.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            image: texture_assets.title_texture.clone().into(),
            ..Default::default()
        })
        .insert(TitleImage);
    commands
        .spawn_bundle(ButtonBundle {
            image: texture_assets.button_texture.clone().into(),
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(330.0),
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
                        left: Val::Px(20.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text {
                    sections: vec![TextSection {
                        value: "Play".to_string(),
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

    // commands.spawn_bundle(ImageBundle {
    //     image: texture_assets.stage_texture.clone().into(),
    //     ..Default::default()
    // });
}

type ButtonInteraction<'a> = (Entity, &'a Interaction, &'a Children);

fn click_play_button(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    q_interaction: Query<ButtonInteraction, (Changed<Interaction>, With<Button>)>,
    q_text: Query<Entity, With<Text>>,
    q_image: Query<Entity, With<TitleImage>>,
) {
    let image = q_image.single();
    for (button, interaction, children) in q_interaction.iter() {
        let text = q_text.get(children[0]).unwrap();
        if *interaction == Interaction::Clicked {
            commands.entity(button).despawn();
            commands.entity(text).despawn();
            commands.entity(image).despawn();
            state.set(GameState::Playing).unwrap();
        }
    }
}
