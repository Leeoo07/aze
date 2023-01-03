use crate::TestDb;
use chrono::NaiveDateTime;
use diesel::RunQueryDsl;
use aze::{
    database::MyJsonType,
    models::{Frame, NewFrame},
    schema::frames as schema_frames,
};
use uuid::Uuid;

mod add;
mod edit;
mod log;
mod start;
mod status;
mod stop;
mod frames;
mod projects;
mod remove;

pub fn add_frame(
    test_db: &TestDb,
    project: &str,
    from: &NaiveDateTime,
    to: Option<&NaiveDateTime>,
    tags: Option<Vec<String>>,
) -> Result<(), Box<dyn std::error::Error>> {
    use serde_json::json;

    let uuid: Uuid = Uuid::new_v4();
    let tags: MyJsonType = MyJsonType(json!(tags));

    let new_frame = NewFrame {
        id: &uuid.to_string(),
        start: from,
        end: to,
        last_update: &NaiveDateTime::default(),
        project,
        tags: &tags,
        deleted: &false,
    };
    let mut conn = test_db.conn().expect("error");
    diesel::insert_into(schema_frames::table)
        .values(&new_frame)
        .execute(&mut conn)
        .expect("Error saving new frame");

    Ok(())
}

pub fn get_frames(test_db: &TestDb) -> Vec<Frame> {
    let mut conn = test_db.conn().expect("error");
    use aze::schema::frames::dsl::*;

    frames.load::<Frame>(&mut conn).expect("error")
}
