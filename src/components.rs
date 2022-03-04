use bevy::prelude::{Bundle, Component, SpriteBundle};
use bevy_rapier2d::prelude::ColliderBundle;
use bevy_rapier2d::prelude::RigidBodyBundle;

#[derive(Bundle)]
pub struct ObjectBundle {
    #[bundle]
    pub rigid_body: RigidBodyBundle,
    #[bundle]
    pub collider: ColliderBundle,
    #[bundle]
    pub sprite: SpriteBundle,
}

#[derive(Component)]
pub struct Dash {
    pub speed: f32,
    pub is_dashing: bool,
    pub duration: f32,
}

// main entity markers
#[derive(Component)]
pub struct Player;

// stats
#[derive(Component)]
pub struct Speed(pub f32);
