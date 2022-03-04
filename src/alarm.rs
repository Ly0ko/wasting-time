use crate::clock::Clock;
use crate::components::{ObjectBundle, Player, Speed};
use crate::grandfather::Grandfather;
use crate::reflector::Reflector;
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{loading::TextureAssets, GameState};

#[derive(Component)]
pub struct Alarm;

#[derive(Component)]
struct Reflected(pub bool);

#[derive(Bundle)]
struct AlarmBundle {
    reflected: Reflected,
    speed: Speed,
    _alarm: Alarm,

    #[bundle]
    object: ObjectBundle,
}

pub struct SpawnAlarmEvent(pub Vec3);

pub struct AlarmPlugin;

impl Plugin for AlarmPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnAlarmEvent>()
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(spawn_alarm)
                    .with_system(move_alarm)
                    .with_system(handle_collisions),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_run_criteria(FixedTimestep::step(10.0))
                    .with_system(cleanup_alarms),
            );
    }
}

fn spawn_alarm(
    mut ev_spawn_alarm: EventReader<SpawnAlarmEvent>,
    mut commands: Commands,
    textures: Res<TextureAssets>,
) {
    for ev in ev_spawn_alarm.iter() {
        let x = ev.0.x;
        let y = ev.0.y;
        commands
            .spawn_bundle(AlarmBundle {
                reflected: Reflected(false),
                speed: Speed(400.0),
                object: ObjectBundle {
                    rigid_body: RigidBodyBundle {
                        body_type: RigidBodyType::Dynamic.into(),
                        mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
                        position: Vec2::new(x, y).into(),
                        ..Default::default()
                    },
                    collider: ColliderBundle {
                        shape: ColliderShape::cuboid(20.0, 25.0).into(),
                        collider_type: ColliderType::Solid.into(),
                        flags: (ActiveEvents::CONTACT_EVENTS).into(),
                        ..Default::default()
                    },
                    sprite: SpriteBundle {
                        texture: textures.alarm_texture.clone(),
                        transform: Transform {
                            translation: Vec3::new(x, y, 0.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                _alarm: Alarm,
            })
            .insert(ColliderPositionSync::Discrete);
    }
}

fn move_alarm(
    mut q_alarm: Query<
        (
            &Speed,
            &Reflected,
            &RigidBodyPositionComponent,
            &mut RigidBodyVelocityComponent,
        ),
        With<Alarm>,
    >,
    q_player: Query<&RigidBodyPositionComponent, With<Player>>,
    rapier_parameters: Res<RapierConfiguration>,
) {
    let player = q_player.single();
    for (speed, reflected, rb_pos, mut rb_vel) in q_alarm.iter_mut() {
        let player_pos = player.position.translation.vector;
        let alarm_pos = rb_pos.position.translation.vector;
        let mut move_delta = player_pos - alarm_pos;
        move_delta /= move_delta.magnitude() * rapier_parameters.scale;

        if reflected.0 {
            rb_vel.linvel = -move_delta * speed.0;
        } else {
            rb_vel.linvel = move_delta * speed.0;
        }
    }
}

fn handle_collisions(
    mut commands: Commands,
    narrow_phase: Res<NarrowPhase>,
    q_player: Query<Entity, With<Player>>,
    q_reflector: Query<Entity, (With<Reflector>, Without<Player>)>,
    mut q_grandfather: Query<(Entity, &mut Clock), With<Grandfather>>,
    mut q_alarm: Query<(Entity, &mut Reflected), With<Alarm>>,
    mut q_clock: Query<&mut Clock, (With<Player>, Without<Grandfather>)>,
) {
    let reflector = q_reflector.single();
    let player = q_player.single();
    for (alarm, mut reflected) in q_alarm.iter_mut() {
        if let Some(contact_pair) = narrow_phase.contact_pair(reflector.handle(), alarm.handle()) {
            if contact_pair.has_any_active_contact {
                reflected.0 = true;
            }
        }
        if let Some(contact_pair) = narrow_phase.contact_pair(player.handle(), alarm.handle()) {
            if contact_pair.has_any_active_contact {
                let mut clock = q_clock.single_mut();
                clock.sub_seconds(30.0);
                commands.entity(alarm).despawn();
            }
        }
        for (grandfather, mut clock) in q_grandfather.iter_mut() {
            if let Some(contact_pair) =
                narrow_phase.contact_pair(grandfather.handle(), alarm.handle())
            {
                if contact_pair.has_any_active_contact {
                    clock.sub_seconds(30.0);
                    commands.entity(alarm).despawn();
                }
            }
        }
    }
}

fn cleanup_alarms(mut commands: Commands, q_alarm: Query<(Entity, &Reflected), With<Alarm>>) {
    for (alarm, reflected) in q_alarm.iter() {
        if reflected.0 {
            commands.entity(alarm).despawn();
        }
    }
}
