use std::ops::Add;

use chrono::{Duration, Local, NaiveDate};

use crate::models::Frame;

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

    pub fn insert_frame(&mut self, frame: Frame) {
        self.frames.insert(0, frame);
    }

    pub fn total_duration(&mut self) -> Duration {
        let mut duration = Duration::zero();
        let frames = self.frames.clone();
        for frame in frames {
            if frame.end.is_some() {
                duration = duration.add(frame.end.unwrap() - frame.start);
            } else {
                let now = Local::now().naive_local();
                duration = duration.add(now - frame.start)
            }
        }

        duration
    }
}

#[cfg(test)]
mod tests {
    use crate::{database::MyJsonType, models::Frame};
    use chrono::{Local, NaiveDate, NaiveDateTime};

    use super::Display;

    #[test]
    fn can_add_multiple_frames() {
        use serde_json::json;

        let frame1 = Frame {
            id: "1".to_string(),
            start: Local::now().naive_local(),
            end: Some(Local::now().naive_local()),
            last_update: Local::now().naive_local(),
            project: "1".to_string(),
            tags: MyJsonType(json!({})),
            deleted: false,
        };

        let frame2 = Frame {
            id: "2".to_string(),
            start: Local::now().naive_local(),
            end: Some(Local::now().naive_local()),
            last_update: Local::now().naive_local(),
            project: "1".to_string(),
            tags: MyJsonType(json!({})),
            deleted: false,
        };

        let display = Display {
            date: Local::now().date().naive_local(),
            frames: vec![frame1, frame2],
        };

        assert_eq!(display.frames.len(), 2);
    }

    #[test]
    fn duration_from_multiple_frames() {
        use serde_json::json;
        let start: NaiveDateTime = NaiveDate::from_ymd(2001, 1, 1).and_hms(10, 0, 0);
        let end: NaiveDateTime = NaiveDate::from_ymd(2001, 1, 1).and_hms(11, 0, 0);
        let frame1 = Frame {
            id: "1".to_string(),
            start,
            end: Some(end),
            last_update: Local::now().naive_local(),
            project: "1".to_string(),
            tags: MyJsonType(json!({})),
            deleted: false,
        };

        let frame2 = Frame {
            id: "2".to_string(),
            start,
            end: Some(end),
            last_update: Local::now().naive_local(),
            project: "1".to_string(),
            tags: MyJsonType(json!({})),
            deleted: false,
        };

        let mut display = Display {
            date: Local::now().date().naive_local(),
            frames: vec![frame1, frame2],
        };

        assert_eq!(2, display.total_duration().num_hours());
    }
}
