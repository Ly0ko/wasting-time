use crate::clock::Clock;
use crate::components::{Dash, ObjectBundle, Player, Speed};
use crate::loading::FontAssets;
use crate::loading::TextureAssets;
use crate::reflector::{spawn_reflector, ReflectorPlugin, ReflectorToggle};
use crate::GameState;

use bevy::prelude::*;
use bevy_rapier2d::na::Vector2;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct PlayerClockUI;

#[derive(Bundle)]
struct PlayerBundle {
    clock: Clock,
    dash: Dash,
    speed: Speed,
    _player: Player,

    #[bundle]
    object: ObjectBundle,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ReflectorPlugin)
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(spawn_player)
                    .with_system(spawn_camera)
                    .with_system(spawn_clock_ui),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(move_player)
                    .with_system(toggle_reflector)
                    .with_system(handle_game_over)
                    .with_system(update_clock_ui),
            );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

// fn spawn_background(mut commands: Commands, textures: Res<TextureAssets>) {
//     commands.spawn_bundle(SpriteBundle {
//         texture: textures.stage_texture.clone(),
//         transform: Transform {
//             translation: Vec3::new(0.0, 0.0, 0.0),
//             ..Default::default()
//         },
//         ..Default::default()
//     });
// }

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    let player_time = 60.0 * 5.0;
    commands
        .spawn_bundle(PlayerBundle {
            clock: Clock::new(player_time),
            dash: Dash {
                speed: 1200.0,
                is_dashing: false,
                duration: 0.1,
            },
            speed: Speed(300.0),
            object: ObjectBundle {
                rigid_body: RigidBodyBundle {
                    body_type: RigidBodyType::Dynamic.into(),
                    mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
                    position: Vec2::new(0.0, 0.0).into(),
                    velocity: RigidBodyVelocity {
                        linvel: Vec2::new(0.0, 0.0).into(),
                        angvel: 0.0,
                    }
                    .into(),
                    ..Default::default()
                },
                collider: ColliderBundle {
                    shape: ColliderShape::cuboid(5.0, 15.0).into(),
                    collider_type: ColliderType::Solid.into(),
                    ..Default::default()
                },
                sprite: SpriteBundle {
                    texture: textures.player_texture_64.clone(),
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, 1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            },
            _player: Player,
        })
        .with_children(|parent| {
            parent.spawn_bundle(spawn_reflector(textures));
        })
        .insert(ColliderPositionSync::Discrete);
}

fn toggle_reflector(
    keyboard_input: Res<Input<KeyCode>>,
    mut ev_reflector_toggle: EventWriter<ReflectorToggle>,
) {
    if keyboard_input.pressed(KeyCode::LShift) {
        ev_reflector_toggle.send(ReflectorToggle(true));
    } else {
        ev_reflector_toggle.send(ReflectorToggle(false));
    }
}

fn move_player(
    window: Res<WindowDescriptor>,
    keyboard_input: Res<Input<KeyCode>>,
    rapier_parameters: Res<RapierConfiguration>,
    time: Res<Time>,
    mut q_player: Query<
        (
            &Speed,
            &mut Dash,
            &RigidBodyPositionComponent,
            &mut RigidBodyVelocityComponent,
        ),
        With<Player>,
    >,
) {
    for (speed, mut dash, rb_pos, mut rb_vels) in q_player.iter_mut() {
        let up = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
        let down = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
        let left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
        let right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);
        let dashing = keyboard_input.just_pressed(KeyCode::Space) && (up || down || left || right);

        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        let mut move_delta = Vector2::new(x_axis as f32, y_axis as f32);
        if move_delta != Vector2::zeros() {
            move_delta /= move_delta.magnitude() * rapier_parameters.scale;
        }

        if dash.is_dashing {
            rb_vels.linvel = move_delta * dash.speed;
            dash.duration -= time.delta_seconds();
            if dash.duration <= 0.0 {
                dash.is_dashing = false;
                dash.duration = 0.1;
            }
        } else if dashing {
            rb_vels.linvel = move_delta * dash.speed;
            dash.is_dashing = true;
        } else {
            rb_vels.linvel = move_delta * speed.0;
        }

        let position = rb_pos.position.translation.vector.data.0[0];
        let bounds_x = window.width as f32 / 2.0;
        let bounds_y = window.height as f32 / 2.0;
        let x = position[0];
        let y = position[1];

        if x < bounds_x - window.width + 20.0 && left {
            rb_vels.linvel.data.0[0][0] = 0.0;
        }
        if x > bounds_x - 20.0 && right {
            rb_vels.linvel.data.0[0][0] = 0.0;
        }
        if y < bounds_y - window.height + 32.0 && down {
            rb_vels.linvel.data.0[0][1] = 0.0;
        }
        if y > bounds_y - 32.0 && up {
            rb_vels.linvel.data.0[0][1] = 0.0;
        }
    }
}

fn handle_game_over(
    mut commands: Commands,
    q_player: Query<&Clock, With<Player>>,
    q_entities: Query<Entity>,
    mut state: ResMut<State<GameState>>,
) {
    let player_clock = q_player.single();
    if player_clock.time <= 0.0 {
        for entity in q_entities.iter() {
            commands.entity(entity).despawn_recursive();
        }
        state.set(GameState::GameOver).unwrap();
    }
}

fn spawn_clock_ui(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Percent(45.0),
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
        .insert(PlayerClockUI);
}

fn update_clock_ui(
    mut q_text: Query<&mut Text, With<PlayerClockUI>>,
    q_clock: Query<&Clock, With<Player>>,
) {
    let clock = q_clock.single();
    let mut text = q_text.single_mut();
    text.sections[0].value = clock.to_string();
}
