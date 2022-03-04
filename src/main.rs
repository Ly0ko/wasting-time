// disable console opening on windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::{App, ClearColor, Color, DefaultPlugins, WindowDescriptor};
use wasting_time::GamePlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Wasting Time".to_string(),
            width: 800.,
            height: 600.,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}
