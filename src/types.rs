use bevy::{
    audio::AudioSource,
    input::{keyboard::KeyCode, Input},
    prelude::{AssetServer, Handle},
};
use serde::{Deserialize, Serialize};

use crate::consts::*;

use core::f32::consts::PI;
use std::{fs::File, io::Read, path::Path};

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Directions {
    Up,
    Down,
    Left,
    Right,
}

impl Directions {
    /// Checks if a key that corresponds to this direction has been pressed
    pub fn key_just_pressed(&self, input: &Input<KeyCode>) -> bool {
        let keys = match self {
            Directions::Up => [KeyCode::Up, KeyCode::W],
            Directions::Down => [KeyCode::Down, KeyCode::S],
            Directions::Left => [KeyCode::Left, KeyCode::A],
            Directions::Right => [KeyCode::Right, KeyCode::D],
        };

        keys.iter().any(|code| input.just_pressed(*code))
    }

    /// Returns the correct rotation for an arrow with this direction
    pub fn rotation(&self) -> f32 {
        match self {
            Directions::Up => PI * 0.5,
            Directions::Down => -PI * 0.5,
            Directions::Left => PI,
            Directions::Right => 0.,
        }
    }

    /// Returns the correct y coordinate for an arrow ith this direction
    pub fn y(&self) -> f32 {
        match self {
            Directions::Up => 150.,
            Directions::Down => 50.,
            Directions::Left => -50.,
            Directions::Right => -150.,
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum Speed {
    Slow,
    Medium,
    Fast,
}

impl Speed {
    pub fn value(&self) -> f32 {
        BASE_SPEED * self.multiplier()
    }

    pub fn multiplier(&self) -> f32 {
        match self {
            Speed::Slow => 1.,
            Speed::Medium => 1.2,
            Speed::Fast => 1.5,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ArrowTime {
    pub spawn_time: f64,
    pub speed: Speed,
    pub direction: Directions,
}

impl ArrowTime {
    fn new(
        ArrowTimeToml {
            speed,
            click_time,
            direction,
        }: &ArrowTimeToml,
    ) -> Self {
        let speed_value = speed.value();
        Self {
            spawn_time: click_time - (DISTANCE / speed_value) as f64,
            speed: *speed,
            direction: *direction,
        }
    }
}

#[derive(Debug)]
pub struct SongConfig {
    pub name: String,
    pub song_audio: Handle<AudioSource>,
    pub arrows: Vec<ArrowTime>,
}

pub fn load_config<P: AsRef<Path> + std::fmt::Display>(
    path: P,
    asset_server: &AssetServer,
) -> SongConfig {
    let mut file = File::open(format!("assets/songs/{}", path)).expect("Couldn't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Couldn't read file into String");

    let parsed: SongConfigToml =
        toml::from_str(&contents).expect("Couldn't parse into SOngConfigToml");

    let mut arrows: Vec<_> = parsed
        .arrows
        .iter()
        .map(|arr| ArrowTime::new(arr))
        .collect();

    arrows.sort_by(|a, b| a.spawn_time.partial_cmp(&b.spawn_time).unwrap());

    let song_audio = asset_server.load(&*format!("songs/{}", parsed.filename));

    SongConfig {
        name: parsed.name,
        song_audio,
        arrows,
    }
}

#[derive(Deserialize, Debug)]
pub struct SongConfigToml {
    pub name: String,
    pub filename: String,
    pub arrows: Vec<ArrowTimeToml>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ArrowTimeToml {
    pub click_time: f64,
    pub speed: Speed,
    pub direction: Directions,
}
