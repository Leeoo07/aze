use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::models::Frame;

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonFrame {
    pub start: NaiveDateTime,
    pub end: Option<NaiveDateTime>,
    pub project: String,
    pub tags: Vec<String>,
}

impl JsonFrame {
    pub fn new(frame: &Frame) -> Self {
        Self {
            start: frame.start,
            end: frame.end,
            project: frame.to_owned().project,
            tags: frame.tags.values(),
        }
    }
}
