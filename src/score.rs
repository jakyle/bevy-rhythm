use crate::consts::*;

#[derive(Default)]
pub struct ScoreResource {
    corrects: usize,
    fails: usize,
    score: usize,
}

impl ScoreResource {
    pub fn increase_correct(&mut self, distance: f32) -> usize {
        self.corrects += 1;

        let score_multiplier = (THRESHOLD - distance.abs()) / THRESHOLD;

        let points = (score_multiplier * 100.).clamp(10., 100.) as usize;
        self.score += points;

        points
    }

    pub fn increase_fails(&mut self) {
        self.fails += 1;
    }

    pub fn score(&self) -> usize {
        self.score
    }

    pub fn corrects(&self) -> usize {
        self.corrects
    }

    pub fn fails(&self) -> usize {
        self.fails
    }
}
