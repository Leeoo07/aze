use crate::database::{establish_connection, MyJsonType};
use crate::models::{Frame, NewFrame};
use crate::schema::frames;
use chrono::NaiveDateTime;

use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;

use uuid::Uuid;

pub fn frame_start_collides(start_b: &NaiveDateTime) -> bool {
    use crate::schema::frames::dsl::*;

    let mut conn = establish_connection();

    let results = frames
        .filter(deleted.eq(false))
        .filter(end.gt(start_b))
        .order_by(start.desc())
        .load::<Frame>(&mut conn)
        .expect("Error loading frames");

    !results.is_empty()
}

pub fn frame_collides(start_b: &NaiveDateTime, end_b: &NaiveDateTime) -> bool {
    use crate::schema::frames::dsl::*;

    let mut conn = establish_connection();
    let results = frames
        .filter(deleted.eq(false))
        .filter(start.lt(end_b))
        .filter(end.gt(start_b))
        .order_by(start.desc())
        .load::<Frame>(&mut conn)
        .expect("Error loading frames");

    !results.is_empty()
}

pub fn create_frame(start: &NaiveDateTime, end: &NaiveDateTime, project: &str, tags: Vec<String>) {
    use serde_json::json;

    let uuid: Uuid = Uuid::new_v4();
    let end_value: Option<&NaiveDateTime> = Option::Some(end);

    let tags: MyJsonType = MyJsonType(json!(tags));

    let mut conn = establish_connection();

    let new_frame = NewFrame {
        id: &uuid.to_string(),
        start,
        end: end_value,
        last_update: &NaiveDateTime::default(),
        project,
        tags: &tags,
        deleted: &false,
    };

    diesel::insert_into(frames::table)
        .values(&new_frame)
        .execute(&mut conn)
        .expect("Error saving new frame");
}

pub fn start_frame(start: &NaiveDateTime, project: &str, tags: Vec<String>) {
    use serde_json::json;

    let uuid: Uuid = Uuid::new_v4();

    let end: Option<&NaiveDateTime> = Option::None;

    let tags: MyJsonType = MyJsonType(json!(tags));
    let mut conn = establish_connection();

    let new_frame = NewFrame {
        id: &uuid.to_string(),
        start,
        end,
        last_update: &NaiveDateTime::default(),
        project,
        tags: &tags,
        deleted: &false,
    };

    diesel::insert_into(frames::table)
        .values(&new_frame)
        .execute(&mut conn)
        .expect("Error saving new frame");
}

pub fn last_started_frame() -> Option<Frame> {
    use crate::schema::frames::dsl::*;
    let mut conn = establish_connection();

    frames
        .filter(deleted.eq(false))
        .filter(end.is_null())
        .order_by(start.desc())
        .load::<Frame>(&mut conn)
        .expect("Error loading frames")
        .pop()
}

pub fn last_finished_frame() -> Option<Frame> {
    use crate::schema::frames::dsl::*;
    use std::collections::VecDeque;
    let mut conn = establish_connection();

    let results = frames
        .filter(deleted.eq(false))
        .filter(end.is_not_null())
        .order_by(end.desc())
        .load::<Frame>(&mut conn)
        .expect("Error loading frames");

    VecDeque::from_iter(results).pop_front()
}
