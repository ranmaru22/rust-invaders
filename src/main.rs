mod enemies;
mod player;
mod ui;

use bevy::prelude::*;

use enemies::EnemyPlugin;
use player::PlayerPlugin;
use ui::{HighScore, UiPlugin, WinSize};

const TIME_STEP: f32 = 1.0 / 60.0;
const RESPAWN_DELAY: f64 = 1.0;

// -- Resources --
struct Materials {
    player: Color,
    bullet: Color,
    e_bullet: Color,
    enemy: Color,
    bonus: Color,
}

// -- Components --
#[derive(Component, Debug)]
struct Speed(f32);
impl Default for Speed {
    fn default() -> Self {
        Self(200.0)
    }
}

fn setup(mut commands: Commands, mut windows: ResMut<Windows>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let window = windows.get_primary_mut().unwrap();

    commands.insert_resource(Materials {
        player: Color::rgb(0.24, 0.4, 0.8),
        bullet: Color::rgb(0.6, 0.6, 0.96),
        e_bullet: Color::rgb(0.8, 0.4, 0.64),
        enemy: Color::rgb(0.8, 0.2, 0.26),
        bonus: Color::rgb(0.9, 0.8, 0.4),
    });

    commands.insert_resource(WinSize {
        w: window.width(),
        h: window.height(),
    });
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(WindowDescriptor {
            title: "Rust Invaders".to_owned(),
            width: 480.0,
            height: 640.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(UiPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_startup_system(setup)
        .run();
}
