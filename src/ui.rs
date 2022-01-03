use crate::ScoreResource;
use bevy::{core::FixedTimestep, prelude::*};
pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_ui.system())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.045))
                    .with_system(update_time_text.system()),
            )
            .add_system(update_score_text.system());
    }
}

fn setup_ui(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let material = color_materials.add(Color::NONE.into());
    let color = Color::rgb(0.8, 0.8, 0.8);
    let font_size = 40.0;
    let font_margin = 10.;

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(font_margin),
                    top: Val::Px(font_margin),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: material.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        TimeText::get_text(0.),
                        TextStyle {
                            font: font.clone(),
                            font_size,
                            color,
                            ..Default::default()
                        },
                        TextAlignment::default(),
                    ),
                    ..Default::default()
                })
                .insert(TimeText);
        });

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(font_margin),
                    top: Val::Px(font_margin + font_size),
                    ..Default::default()
                },
                ..Default::default()
            },
            material,
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        ScoreText::get_text(0, 0, 0),
                        TextStyle {
                            font: font.clone(),
                            font_size,
                            color,
                            ..Default::default()
                        },
                        TextAlignment::default(),
                    ),
                    ..Default::default()
                })
                .insert(ScoreText);
        });
}

fn update_time_text(time: Res<Time>, mut query: Query<&mut Text, With<TimeText>>) {
    let secs = time.seconds_since_startup() - 3.;

    if secs < 0. {
        return;
    }

    for mut text in query.iter_mut() {
        if let Some(ts) = try_get_text_section(&mut text, "Time") {
            ts.value = TimeText::get_text(secs);
        }
    }
}

struct ScoreText;

impl ScoreText {
    pub fn get_text<T: std::fmt::Display>(score: T, corrects: T, fails: T) -> String {
        format!("Score: {}, Corrects: {}, Fails: {}", score, corrects, fails)
    }
}

fn update_score_text(score: Res<ScoreResource>, mut query: Query<&mut Text, With<ScoreText>>) {
    if score.is_changed() {
        for mut text in query.iter_mut() {
            if let Some(ts) = try_get_text_section(&mut text, "Score") {
                ts.value = ScoreText::get_text(score.score(), score.corrects(), score.fails());
            }
        }
    }
}

fn try_get_text_section<'a>(text: &'a mut Mut<Text>, pattern: &str) -> Option<&'a mut TextSection> {
    text.sections
        .iter_mut()
        .find(|ts| ts.value.contains(pattern))
}

struct TimeText;
impl TimeText {
    pub fn get_text<T: std::fmt::Display>(secs: T) -> String {
        format!("Time: {:.2}", secs)
    }
}
