mod player;
mod enemies;
mod ui;

use std::collections::HashSet;

use bevy::{prelude::*, sprite::collide_aabb::collide};
use player::PlayerPlugin;
use enemies::EnemyPlugin;
use ui::UiPlugin;

const TIME_STEP: f32 = 1.0 / 60.0;
const RESPAWN_DELAY: f64 = 1.0;

// -- Resources --
struct Materials {
    player: Color,
    bullet: Color,
    e_bullet: Color,
    enemy: Color,
}

struct WinSize {
    #[allow(unused)]
    w: f32,
    h: f32,
}

struct PlayerState {
    alive: bool,
    last_death: f64,
}

impl PlayerState {
    fn spawn(&mut self) {
        self.alive = true;
    }

    fn kill_at_time(&mut self, time: f64) {
        self.alive = false;
        self.last_death = time;
    }
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            alive: false,
            last_death: 0.0,
        }
    }
}
// -- End Resources --

// -- Components --
#[derive(Component)]
struct Player;
#[derive(Component)]
struct PlayerReadyFire(bool);

#[derive(Component)]
struct Bullet;

#[derive(Component)]
struct Enemy;
#[derive(Component)]
struct ActiveEnemies(u32);

#[derive(Component)]
struct EnemyBullet;

#[derive(Component, Debug)]
struct Speed(f32);
impl Default for Speed {
    fn default() -> Self {
        Self(200.0)
    }
}

#[derive(Component)]
struct HighScoreDisplay;
#[derive(Component)]
struct ScoreDisplay;
#[derive(Component)]
struct HighScore(u32, u32);
// -- End Components --

// -- Systems --
fn setup(
    mut commands: Commands,
    mut windows: ResMut<Windows>
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // DEBUG: Put the window on the right so it doesn't overlap the editor
    let window = windows.get_primary_mut().unwrap();
    window.set_position(IVec2::new(1800, 50));
    // END DEBUG

    commands.insert_resource(Materials {
        player: Color::rgb(0.8, 0.0, 0.0),
        bullet: Color::rgb(0.6, 0.6, 0.96),
        e_bullet: Color::rgb(0.8, 0.4, 0.64),
        enemy: Color::rgb(0.8, 0.2, 0.26),
    });

    commands.insert_resource(WinSize {
        w: window.width(),
        h: window.height(),
    });
}

fn bullet_hit(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform, &Sprite), With<Bullet>>,
    enemy_query: Query<(Entity, &Transform, &Sprite), With<Enemy>>,
    mut score: ResMut<HighScore>,
    mut active_enemies: ResMut<ActiveEnemies>
) {
    let mut kills: HashSet<Entity> = HashSet::new();

    for (bullet, bullet_tf, bullet_sprite) in bullet_query.iter() {
        for (enemy, enemy_tf, enemy_sprite) in enemy_query.iter() {
            let bullet_scale = bullet_tf.scale.abs().truncate();
            let enemy_scale = enemy_tf.scale.abs().truncate();

            if let Some(_collision) = collide(
                bullet_tf.translation,
                bullet_sprite.custom_size.unwrap() * bullet_scale,
                enemy_tf.translation,
                enemy_sprite.custom_size.unwrap() * enemy_scale
            ) {
                if kills.get(&enemy).is_none() {
                    // Despawn colliding sprites
                    commands.entity(enemy).despawn();
                    commands.entity(bullet).despawn();

                    score.0 += 100;
                    active_enemies.0 -= 1;

                    kills.insert(enemy);
                }
            };
        }
    }
}

fn e_bullet_hit(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    time: Res<Time>,
    bullet_query: Query<(Entity, &Transform, &Sprite), With<EnemyBullet>>,
    mut player_query: Query<(Entity, &Transform, &Sprite), With<Player>>
) {
    if let Ok((player, player_tf, player_sprite)) = player_query.get_single_mut() {
        let player_scale = player_tf.scale.abs().truncate();

        for (bullet, bullet_tf, bullet_sprite) in bullet_query.iter() {
            let bullet_scale = bullet_tf.scale.abs().truncate();

            if let Some(_collision) = collide(
                bullet_tf.translation,
                bullet_sprite.custom_size.unwrap() * bullet_scale,
                player_tf.translation,
                player_sprite.custom_size.unwrap() * player_scale
            ) {
                // Kill!
                commands.entity(bullet).despawn();
                commands.entity(player).despawn();

                player_state.kill_at_time(time.seconds_since_startup());
            };
    }
    }
}

type ScoreQ<'a> = QueryState<&'a mut Text, With<ScoreDisplay>>;
type HiScoreQ<'a> = QueryState<&'a mut Text, With<HighScoreDisplay>>;

fn draw_score(
    time: Res<Time>,
    mut query: QuerySet<(ScoreQ, HiScoreQ)>,
    score: Res<HighScore>
) {
    for mut text in query.q0().iter_mut() {
        text.sections[1].value = score.0.to_string();
    }

    for mut text in query.q1().iter_mut() {
        let seconds = time.seconds_since_startup() as f32;
        text.sections[1].value = score.1.to_string();

        text.sections[1].style.color = Color::Rgba {
            red: (1.25 * seconds).sin() / 2.0 + 0.5,
            green: (0.75 * seconds).sin() / 2.0 + 0.5,
            blue: (0.50 * seconds).sin() / 2.0 + 0.5,
            alpha: 1.0,
        };
    }
}
// -- End Systems --

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(WindowDescriptor {
            title: "Rust Invaders".to_owned(),
            width: 480.0,
            height: 640.0 ,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(UiPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_startup_system(setup)
        .add_system(bullet_hit)
        .add_system(e_bullet_hit)
        .add_system(draw_score)
        .run();
}
