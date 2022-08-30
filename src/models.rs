use crate::database::MyJsonType;

use super::schema::frames;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
#[derive(Queryable, Clone, Identifiable)]
pub struct Frame {
    pub id: String,
    pub start: NaiveDateTime,
    pub end: Option<NaiveDateTime>,
    pub last_update: NaiveDateTime,
    pub project: String,
    pub tags: MyJsonType,
    pub deleted: bool,
}

#[derive(Insertable)]
#[table_name = "frames"]
pub struct NewFrame<'a> {
    pub id: &'a str,
    pub start: &'a NaiveDateTime,
    pub end: Option<&'a NaiveDateTime>,
    pub last_update: &'a NaiveDateTime,
    pub project: &'a str,
    pub tags: &'a MyJsonType,
    pub deleted: &'a bool,
}
