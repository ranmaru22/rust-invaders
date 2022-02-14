use crate::{
    player::{Player, PlayerState},
    Materials, Speed, WinSize, TIME_STEP,
};
use bevy::{core::FixedTimestep, prelude::*, sprite::collide_aabb::collide};
use rand::{thread_rng, Rng};
use std::f32::consts::PI;

const MAX_ENEMIES: u32 = 3;
const FORMATION_SIZE: u32 = 3;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActiveEnemies(0))
            .insert_resource(FormationMaker::default())
            .add_system(enemy_movement)
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

#[derive(Component, Default, Clone)]
pub struct Formation {
    start: (f32, f32),
    radius: (f32, f32),
    offset: (f32, f32),
    angle: f32,
    #[allow(unused)]
    id: u32,
}

#[derive(Component, Default)]
pub struct FormationMaker {
    seq_id: u32,
    current: Option<Formation>,
    current_members: u32,
}

impl FormationMaker {
    fn make(&mut self, win_size: &Res<WinSize>) -> Formation {
        match (
            self.current.as_ref(),
            self.current_members >= FORMATION_SIZE,
        ) {
            (None, _) | (_, true) => {
                let mut rng = thread_rng();

                let h = win_size.h / 2.0 - 100.0;
                let w = win_size.w / 4.0;

                let x = if rng.gen_bool(0.5) {
                    win_size.w
                } else {
                    -win_size.w
                };

                let y = rng.gen_range(-h..h);

                let start = (x, y);

                let offset = (rng.gen_range(-w..w), rng.gen_range(0.0..h));
                let radius = (rng.gen_range(80.0..150.0), 100.0);
                let angle = (y - offset.0).atan2(x - offset.1);

                self.seq_id += 1;
                let id = self.seq_id;
                let formation = Formation {
                    start,
                    offset,
                    radius,
                    angle,
                    id,
                };

                self.current = Some(formation.clone());
                self.current_members = 1;

                formation
            }

            (Some(formation), _) => {
                self.current_members += 1;
                formation.clone()
            }
        }
    }
}

fn spawn_enemy(
    mut commands: Commands,
    mut active_enemies: ResMut<ActiveEnemies>,
    mut formation_maker: ResMut<FormationMaker>,
    win_size: Res<WinSize>,
    materials: Res<Materials>,
) {
    if active_enemies.0 < MAX_ENEMIES {
        let formation = formation_maker.make(&win_size);
        let (x, y) = formation.start;

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
            .insert(Speed::default())
            .insert(formation);

        active_enemies.0 += 1;
    }
}

fn enemy_movement(mut query: Query<(&mut Transform, &Speed, &mut Formation), With<Enemy>>) {
    for (mut transform, speed, mut formation) in query.iter_mut() {
        let max_dist = TIME_STEP * speed.0;
        let x_origin = transform.translation.x;
        let y_origin = transform.translation.y;

        let (x_offset, y_offset) = formation.offset;
        let (x_radius, y_radius) = formation.radius;

        let dir = if formation.start.0 > 0.0 { 1.0 } else { -1.0 };
        let angle =
            formation.angle + dir * speed.0 * TIME_STEP / (x_radius.min(y_radius) * PI / 2.0);

        let x_dest = x_radius * angle.cos() + x_offset;
        let y_dest = y_radius * angle.sin() + y_offset;

        let (dx, dy) = (x_origin - x_dest, y_origin - y_dest);

        let distance = (dx * dx + dy * dy).sqrt();
        let dist_ratio = if distance == 0.0 {
            0.0
        } else {
            max_dist / distance
        };

        let x = x_origin - dx * dist_ratio;
        let y = y_origin - dy * dist_ratio;

        let x = if dx > 0.0 {
            x.max(x_dest)
        } else {
            x.min(x_dest)
        };

        let y = if dy > 0.0 {
            y.max(y_dest)
        } else {
            y.min(y_dest)
        };

        if distance < max_dist * speed.0 / 20.0 {
            formation.angle = angle;
        }

        transform.translation.x = x;
        transform.translation.y = y;
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
