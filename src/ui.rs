use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(draw_score)
            .add_startup_stage("setup_ui", SystemStage::single(setup_ui));
    }
}

type ScoreQ<'a> = QueryState<&'a mut Text, With<ScoreDisplay>>;
type HiScoreQ<'a> = QueryState<&'a mut Text, With<HighScoreDisplay>>;

// -- Components --
#[derive(Component)]
pub struct HighScoreDisplay;
#[derive(Component)]
pub struct ScoreDisplay;
#[derive(Component)]
pub struct HighScore(pub u32, pub u32);

// -- Resources --
pub struct WinSize {
    #[allow(unused)]
    pub w: f32,
    pub h: f32,
}

fn setup_ui(mut commands: Commands, assets: Res<AssetServer>) {
    let iosevka = assets.load("fonts/iosevka-ran-medium.ttf");
    let font_size = 32.0;

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                justify_content: JustifyContent::SpaceBetween,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },

            text: Text {
                sections: vec![
                    TextSection {
                        value: String::from("Score: "),
                        style: TextStyle {
                            font: iosevka.clone_weak(),
                            font_size,
                            color: Color::YELLOW,
                        },
                    },
                    TextSection {
                        value: 0.to_string(),
                        style: TextStyle {
                            font: iosevka.clone_weak(),
                            font_size,
                            color: Color::WHITE,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ScoreDisplay);

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                justify_content: JustifyContent::SpaceBetween,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },

            text: Text {
                sections: vec![
                    TextSection {
                        value: String::from("Best: "),
                        style: TextStyle {
                            font: iosevka.clone_weak(),
                            font_size,
                            color: Color::YELLOW,
                        },
                    },
                    TextSection {
                        value: 0.to_string(),
                        style: TextStyle {
                            // Last use of the handle, transfer ownership.
                            font: iosevka,
                            font_size,
                            color: Color::WHITE,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(HighScoreDisplay);

    commands.insert_resource(HighScore(0, 0));
}

fn draw_score(time: Res<Time>, mut query: QuerySet<(ScoreQ, HiScoreQ)>, score: Res<HighScore>) {
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
