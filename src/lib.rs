mod alarm;
mod clock;
mod components;
mod game_over;
mod game_win;
mod grandfather;
mod loading;
mod menu;
mod player;
mod reflector;

use alarm::AlarmPlugin;
use bevy::prelude::{App, Plugin};
use bevy_rapier2d::physics::{NoUserData, RapierPhysicsPlugin};
use clock::ClockPlugin;
use game_over::GameOverPlugin;
use game_win::GameWinPlugin;
use grandfather::GrandfatherPlugin;
use loading::LoadingPlugin;
use menu::MenuPlugin;
use player::PlayerPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Loading,
    Menu,
    Playing,
    GameOver,
    GameWin,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(GrandfatherPlugin)
            .add_plugin(ClockPlugin)
            .add_plugin(GameOverPlugin)
            .add_plugin(GameWinPlugin);
    }
}
