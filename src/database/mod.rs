use std::io::Write;

use crate::config::load_config;
use diesel::backend::Backend;
use diesel::serialize::Output;
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use diesel::types::{FromSql, ToSql};
use diesel::{deserialize, serialize};
use diesel::{Connection, SqliteConnection};

pub fn establish_connection() -> SqliteConnection {
    let cfg = load_config();

    if cfg.is_err() {
        panic!("Config could not be loaded");
    }
    let path = cfg.unwrap().data_dir + &std::path::MAIN_SEPARATOR.to_string() + "frames.db";

    return SqliteConnection::establish(&path).expect(&format!("Error connecting to {}", &path));
}

#[derive(Debug, AsExpression, FromSqlRow, Clone, PartialEq, Eq)]
#[sql_type = "Text"]
pub struct MyJsonType(pub serde_json::Value);

impl FromSql<Text, Sqlite> for MyJsonType {
    fn from_sql(
        bytes: Option<&<diesel::sqlite::Sqlite as Backend>::RawValue>,
    ) -> deserialize::Result<Self> {
        let t = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
        Ok(Self(serde_json::from_str(&t)?))
    }
}
impl ToSql<Text, Sqlite> for MyJsonType {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Sqlite>) -> serialize::Result {
        let s = serde_json::to_string(&self.0)?;
        <String as ToSql<Text, Sqlite>>::to_sql(&s, out)
    }
}
