use std::ops::Add;

use chrono::{Duration, NaiveDate};
use mycroft::models::Frame;

#[derive(Clone)]
pub struct Display {
    pub date: NaiveDate,
    pub frames: Vec<Frame>,
}

impl Display {
    pub fn new(date: NaiveDate, frames: Vec<Frame>) -> Self {
        Self { date, frames }
    }

    pub fn add_frame(&mut self, frame: Frame) {
        self.frames.push(frame);
    }

    pub fn total_duration(&mut self) -> Duration {
        let mut duration = Duration::zero();
        let frames = self.frames.clone();
        for frame in frames {
            if frame.end.is_some() {
                duration = duration.add(frame.end.unwrap() - frame.start);
            }
        }

        return duration;
    }
}
