use bevy::prelude::*;
use crate::{HighScore, ScoreDisplay, HighScoreDisplay};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_stage(
            "setup_ui",
            SystemStage::single(setup_ui)
        );
    }
}

fn setup_ui(
    mut commands: Commands,
    assets: Res<AssetServer>
) {
    let iosevka = assets.load("fonts/iosevka-ran-medium.ttf");
    let font_size = 32.0;

    commands.spawn_bundle(TextBundle {
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
                }
            ],
            ..Default::default()
        },
        ..Default::default()
    })
        .insert(ScoreDisplay);

    commands.spawn_bundle(TextBundle {
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
                }
            ],
            ..Default::default()
        },
        ..Default::default()
    })
        .insert(HighScoreDisplay);

    commands.insert_resource(HighScore(0, 0));
}
