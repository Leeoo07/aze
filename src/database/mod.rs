use std::error::Error;

use crate::config::load_config;
use diesel::backend::{Backend, RawValue};
use diesel::deserialize::FromSql;
use diesel::r2d2;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use diesel::SqliteConnection;
use diesel::{deserialize, serialize};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub fn get_connection_pool() -> Pool<ConnectionManager<SqliteConnection>> {
    let cfg = load_config();
    let url = cfg.database_url();

    let manager = ConnectionManager::<SqliteConnection>::new(url);
    return r2d2::Pool::new(manager).unwrap();
}

pub fn establish_connection() -> PooledConnection<ConnectionManager<SqliteConnection>> {
    let pool = get_connection_pool();

    return pool.get().unwrap();
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn run_migrations(
    connection: &mut impl MigrationHarness<diesel::sqlite::Sqlite>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}

#[derive(Debug, AsExpression, FromSqlRow, Clone, PartialEq, Eq)]
#[diesel(sql_type = Text)]
pub struct MyJsonType(pub serde_json::Value);

impl MyJsonType {
    pub fn values(&self) -> Vec<String> {
        if !self.0.is_array() {
            return vec![];
        }

        let array = self.0.as_array().unwrap();
        let mut vec: Vec<String> = vec![];

        for item in array {
            if !item.is_string() {
                continue;
            }

            vec.push(item.as_str().unwrap().trim_matches('"').to_string());
        }
        return vec;
    }
}

impl<DB> FromSql<Text, DB> for MyJsonType
where
    DB: Backend,
    *const str: FromSql<Text, DB>,
{
    fn from_sql(bytes: RawValue<DB>) -> deserialize::Result<Self> {
        let t = <String as FromSql<Text, DB>>::from_sql(bytes)?;
        Ok(Self(serde_json::from_str(&t)?))
    }
}

impl ToSql<Text, Sqlite> for MyJsonType
where
    String: ToSql<Text, Sqlite>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let s = serde_json::to_string(&self.0)?;

        out.set_value(s);
        Ok(IsNull::No)
    }
}
