use crate::{
    player::{Player, PlayerState},
    Materials, Speed, WinSize, TIME_STEP,
};
use bevy::{core::FixedTimestep, prelude::*, sprite::collide_aabb::collide};
use rand::Rng;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActiveEnemies(0))
            .add_system(e_bullet_movement)
            .add_system(e_bullet_hit)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1.0))
                    .with_system(spawn_enemy),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.85))
                    .with_system(enemy_fire),
            );
    }
}

// -- Components --
#[derive(Component)]
pub struct Enemy;
#[derive(Component)]
pub struct ActiveEnemies(pub u32);
#[derive(Component)]
pub struct EnemyBullet;

fn spawn_enemy(
    mut commands: Commands,
    mut active_enemies: ResMut<ActiveEnemies>,
    win_size: Res<WinSize>,
    materials: Res<Materials>,
) {
    if active_enemies.0 < 3 {
        let mut rng = rand::thread_rng();

        let spawn_x = win_size.w / 2.0 - 100.0;
        let spawn_y = win_size.h / 2.0 - 100.0;

        let x = rng.gen_range(-spawn_x..spawn_x);
        let y = rng.gen_range(-spawn_y..spawn_y);

        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(36.0, 12.0)),
                    color: materials.enemy,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(x, y, 10.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Enemy)
            .insert(Speed::default());

        active_enemies.0 += 1;
    }
}

fn enemy_fire(
    mut commands: Commands,
    materials: Res<Materials>,
    query: Query<&Transform, With<Enemy>>,
) {
    for transform in query.iter() {
        let x = transform.translation.x;
        let y = transform.translation.y;

        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(2.0, 16.0)),
                    color: materials.e_bullet,
                    ..Default::default()
                },

                transform: Transform {
                    translation: Vec3::new(x, y, 0.0),
                    ..Default::default()
                },

                ..Default::default()
            })
            .insert(EnemyBullet)
            .insert(Speed::default());
    }
}

fn e_bullet_movement(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Speed, &mut Transform), With<EnemyBullet>>,
) {
    for (entity, speed, mut transform) in query.iter_mut() {
        transform.translation.y -= speed.0 * TIME_STEP;

        if transform.translation.y < -win_size.h / 2.0 - 50.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn e_bullet_hit(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    time: Res<Time>,
    bullet_query: Query<(Entity, &Transform, &Sprite), With<EnemyBullet>>,
    mut player_query: Query<(Entity, &Transform, &Sprite), With<Player>>,
) {
    if let Ok((player, player_tf, player_sprite)) = player_query.get_single_mut() {
        let player_scale = player_tf.scale.abs().truncate();

        for (bullet, bullet_tf, bullet_sprite) in bullet_query.iter() {
            let bullet_scale = bullet_tf.scale.abs().truncate();

            if let Some(_collision) = collide(
                bullet_tf.translation,
                bullet_sprite.custom_size.unwrap() * bullet_scale,
                player_tf.translation,
                player_sprite.custom_size.unwrap() * player_scale,
            ) {
                // Kill!
                commands.entity(bullet).despawn();
                commands.entity(player).despawn();

                player_state.kill_at_time(time.seconds_since_startup());
            };
        }
    }
}
