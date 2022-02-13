use bevy::{core::FixedTimestep, prelude::*};
use rand::Rng;
use crate::{Enemy, ActiveEnemies, Materials, WinSize, EnemyBullet, Speed, TIME_STEP};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActiveEnemies(0))
            .add_system(e_bullet_movement)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1.0))
                    .with_system(spawn_enemy)
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.85))
                    .with_system(enemy_fire)
            );
    }
}

fn spawn_enemy(
    mut commands: Commands,
    mut active_enemies: ResMut<ActiveEnemies>,
    win_size: Res<WinSize>,
    materials: Res<Materials>
) {
    if active_enemies.0 < 3 {
        let mut rng = rand::thread_rng();

        let spawn_x = win_size.w / 2.0 - 100.0;
        let spawn_y = win_size.h / 2.0 - 100.0;

        let x = rng.gen_range(-spawn_x..spawn_x);
        let y = rng.gen_range(-spawn_y..spawn_y);

        commands.spawn_bundle(SpriteBundle {
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
    query: Query<&Transform, With<Enemy>>
) {
    for transform in query.iter() {
        let x = transform.translation.x;
        let y = transform.translation.y;

        commands.spawn_bundle(SpriteBundle {
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
    mut query: Query<(Entity, &Speed, &mut Transform), With<EnemyBullet>>
) {
    for (entity, speed, mut transform) in query.iter_mut() {
        transform.translation.y -= speed.0 * TIME_STEP;

        if transform.translation.y < -win_size.h / 2.0 - 50.0 {
            commands.entity(entity).despawn();
        }
    }
}
