use bevy::prelude::*;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::GameState;

#[derive(Component)]
pub struct Clock {
    pub minutes: f32,
    pub seconds: f32,
    pub time: f32,
    pub speed: f32,
}

impl Clock {
    pub fn new(time: f32) -> Self {
        Self {
            minutes: 0.0,
            seconds: 0.0,
            speed: 1.0,
            time,
        }
    }
    pub fn sub_seconds(&mut self, seconds: f32) {
        self.time -= seconds;
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let minutes = self.time as u32 / 60;
        let seconds = self.time as u32 % 60;
        let fmt_time = format!("{:02}:{:02}", minutes, seconds);
        write!(f, "{}", fmt_time)
    }
}

pub struct ClockPlugin;

impl Plugin for ClockPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(tick_clock));
    }
}

fn tick_clock(mut q_clocks: Query<&mut Clock>, time: Res<Time>) {
    for mut clock in q_clocks.iter_mut() {
        clock.time -= clock.speed * time.delta_seconds();
        clock.minutes = clock.time / 60.0;
        clock.seconds = clock.time % 60.0;
    }
}
