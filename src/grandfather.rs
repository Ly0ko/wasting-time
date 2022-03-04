use crate::alarm::SpawnAlarmEvent;
use crate::clock::Clock;
use crate::loading::FontAssets;
use crate::AlarmPlugin;
use bevy::{core::FixedTimestep, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::{loading::TextureAssets, GameState};

#[derive(Component)]
pub struct GrandSunClockUI;

#[derive(Component)]
pub struct GrandMoonClockUI;

#[derive(Component)]
pub struct Grandfather;

#[derive(Component)]
pub struct GrandfatherMoon;

#[derive(Component)]
pub struct GrandfatherSun;

pub struct GrandfatherPlugin;

impl Plugin for GrandfatherPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AlarmPlugin)
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(spawn_grandfather)
                    .with_system(spawn_clock_ui),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_run_criteria(FixedTimestep::step(3.0))
                    .with_system(spawn_alarm),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(handle_game_over)
                    .with_system(update_clock_ui),
            );
    }
}

fn spawn_grandfather(mut commands: Commands, textures: Res<TextureAssets>) {
    let sun_x = -350.0;
    let sun_y = 210.0;
    let moon_x = 350.0;
    let moon_y = 200.0;
    let clock_time = 60.0 * 5.0;

    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.grandfather_sun.clone(),
            transform: Transform {
                translation: Vec3::new(sun_x, sun_y, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(70.0, 80.0).into(),
            collider_type: ColliderType::Solid.into(),
            position: Vec2::new(sun_x, sun_y).into(),
            ..Default::default()
        })
        .insert(Clock::new(clock_time))
        .insert(Grandfather)
        .insert(GrandfatherSun);

    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.grandfather_moon.clone(),
            transform: Transform {
                translation: Vec3::new(moon_x, moon_y, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(90.0, 80.0).into(),
            collider_type: ColliderType::Solid.into(),
            position: Vec2::new(moon_x, moon_y).into(),
            ..Default::default()
        })
        .insert(Clock::new(clock_time))
        .insert(Grandfather)
        .insert(GrandfatherMoon);
}

fn spawn_clock_ui(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(200.0),
                    left: Val::Px(60.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: "00:00".to_string(),
                    style: bevy::text::TextStyle {
                        font: font_assets.roboto.clone(),
                        font_size: 30.0,
                        color: Color::rgb(1., 1., 1.),
                    },
                }],
                alignment: Default::default(),
            },
            ..Default::default()
        })
        .insert(GrandSunClockUI);

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(200.0),
                    right: Val::Px(60.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: "00:00".to_string(),
                    style: bevy::text::TextStyle {
                        font: font_assets.roboto.clone(),
                        font_size: 30.0,
                        color: Color::rgb(1., 1., 1.),
                    },
                }],
                alignment: Default::default(),
            },
            ..Default::default()
        })
        .insert(GrandMoonClockUI);
}

fn spawn_alarm(
    mut ev_spawn_alarm: EventWriter<SpawnAlarmEvent>,
    grandfather_moon: Query<(&Clock, &Transform), With<GrandfatherMoon>>,
    grandfather_sun: Query<(&Clock, &Transform), With<GrandfatherSun>>,
) {
    for (clock, moon) in grandfather_moon.iter() {
        if clock.time > 0.0 {
            let moon_pos = moon.translation;
            let position = Vec3::new(moon_pos.x - 150.0, moon_pos.y - 20.0, 0.0);
            ev_spawn_alarm.send(SpawnAlarmEvent(position));
        }
    }
    for (clock, sun) in grandfather_sun.iter() {
        if clock.time > 0.0 {
            let sun_pos = sun.translation;
            let position = Vec3::new(sun_pos.x + 120.0, sun_pos.y - 20.0, 0.0);
            ev_spawn_alarm.send(SpawnAlarmEvent(position));
        }
    }
}

fn update_clock_ui(
    mut q_sun_text: Query<&mut Text, (With<GrandSunClockUI>, Without<GrandMoonClockUI>)>,
    mut q_moon_text: Query<&mut Text, (With<GrandMoonClockUI>, Without<GrandSunClockUI>)>,
    q_sun_clock: Query<&Clock, With<GrandfatherSun>>,
    q_moon_clock: Query<&Clock, With<GrandfatherMoon>>,
) {
    let sun_clock = q_sun_clock.single();
    let mut text = q_sun_text.single_mut();
    text.sections[0].value = sun_clock.to_string();

    let q_moon_clock = q_moon_clock.single();
    let mut text = q_moon_text.single_mut();
    text.sections[0].value = q_moon_clock.to_string();
}

fn handle_game_over(
    mut commands: Commands,
    q_grandfathers: Query<&Clock, With<Grandfather>>,
    q_entities: Query<Entity>,
    mut state: ResMut<State<GameState>>,
) {
    let mut dead_count = 0;
    for clock in q_grandfathers.iter() {
        if clock.time <= 0.0 {
            dead_count += 1;
        }
    }

    if dead_count == 2 {
        for entity in q_entities.iter() {
            commands.entity(entity).despawn_recursive();
        }
        state.set(GameState::GameWin).unwrap();
    }
}
