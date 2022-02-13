use bevy::{prelude::*, core::FixedTimestep};
use crate::{Materials, WinSize, Player, Speed, PlayerReadyFire, Bullet, TIME_STEP, PlayerState, RESPAWN_DELAY, HighScore};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerState::default())
            .add_startup_stage(
            "spawn_player",
            SystemStage::single(spawn_player)
        )
            .add_system(player_movement)
            .add_system(player_fire)
            .add_system(bullet_movement)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1.0))
                    .with_system(spawn_player)
            );
    }
}

fn spawn_player(
    mut commands: Commands,
    mut state: ResMut<PlayerState>,
    mut score: ResMut<HighScore>,
    time: Res<Time>,
    materials: Res<Materials>,
    win_size: Res<WinSize>
) {
    let now = time.seconds_since_startup();
    let last_death = state.last_death;

    if !state.alive && (last_death == 0.0 || now > last_death + RESPAWN_DELAY) {
        state.spawn();

        if score.0 > score.1 {
            score.1 = score.0;
        }

        score.0 = 0;

        let bottom = -win_size.h / 2.0;
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(10.0, 10.0)),
                color: materials.player,
                ..Default::default()
            },

            transform: Transform {
                translation: Vec3::new(0.0, bottom + 10.0, 10.0),
                ..Default::default()
            },

            ..Default::default()
        })
            .insert(Player)
            .insert(Speed::default())
            .insert(PlayerReadyFire(true));
    }
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Speed, &mut Transform), With<Player>>
) {
    if let Ok((speed, mut transform)) = query.get_single_mut() {
        let dir = if keyboard_input.pressed(KeyCode::A) {
            -1.0
        } else if keyboard_input.pressed(KeyCode::S) {
            1.0
        } else { 0.0 };

        transform.translation.x += dir * speed.0 * TIME_STEP;
    }
}

fn player_fire(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    materials: Res<Materials>,
    mut query: Query<(&Transform, &mut PlayerReadyFire), With<Player>>
){
    if let Ok((transform, mut is_ready)) = query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::Space) && is_ready.0 {
            is_ready.0 = false;

            let x = transform.translation.x;
            let y = transform.translation.y;

            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(2.0, 4.0)),
                    color: materials.bullet,
                    ..Default::default()
                },

                transform: Transform {
                    translation: Vec3::new(x, y, 0.0),
                    ..Default::default()
                },

                ..Default::default()
            })
                .insert(Bullet)
                .insert(Speed::default());
        }

        if keyboard_input.just_released(KeyCode::Space) {
            is_ready.0 = true;
        }
    }
}

fn bullet_movement(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Speed, &mut Transform), With<Bullet>>
) {
    for (entity, speed, mut transform) in query.iter_mut() {
        transform.translation.y += speed.0 * TIME_STEP;

        if transform.translation.y > win_size.h {
            commands.entity(entity).despawn();
        }
    }
}
