use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub enum ReflectorState {
    Active,
    InActive,
}

#[derive(Component)]
pub struct Reflector {
    pub state: ReflectorState,
}

#[derive(Bundle)]
pub struct ReflectorBundle {
    _reflector: Reflector,

    #[bundle]
    collider: ColliderBundle,
    #[bundle]
    sprite: SpriteBundle,
}

pub struct ReflectorToggle(pub bool);

pub struct ReflectorPlugin;

impl Plugin for ReflectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ReflectorToggle>().add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(update_reflector_state),
        );
    }
}

pub fn spawn_reflector(textures: Res<TextureAssets>) -> ReflectorBundle {
    ReflectorBundle {
        collider: ColliderBundle {
            position: Vec2::new(0.0, 40.0).into(),
            shape: ColliderShape::cuboid(2.0, 1.0).into(),
            collider_type: ColliderType::Solid.into(),
            ..Default::default()
        },
        sprite: SpriteBundle {
            texture: textures.reflector_texture.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 40.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        },
        _reflector: Reflector {
            state: ReflectorState::InActive,
        },
    }
}

pub fn update_reflector_state(
    mut ev_reflector_toggle: EventReader<ReflectorToggle>,
    mut q_reflector: Query<&mut Reflector>,
) {
    for toggle in ev_reflector_toggle.iter() {
        let mut reflector = q_reflector.single_mut();
        if toggle.0 {
            // println!("reflector active");
            reflector.state = ReflectorState::Active;
        } else {
            // println!("reflector inactive");
            reflector.state = ReflectorState::InActive;
        }
    }
}
