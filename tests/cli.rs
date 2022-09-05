use diesel::{
    r2d2::{self, ConnectionManager, Pool, PooledConnection},
    SqliteConnection,
};
use mycroft::database::run_migrations;

mod commands;

pub struct TestDb {
    pub pool: Pool<ConnectionManager<SqliteConnection>>,
    pub db_path: String,
    pub tmp_dir: tempfile::TempDir,
}

impl TestDb {
    pub fn new() -> TestDb {
        let tmp_dir = tempfile::Builder::new()
            .prefix(env!("CARGO_PKG_NAME"))
            .rand_bytes(5)
            .tempdir()
            .expect("not possible to create tempfile");

        let db_path = tmp_dir.path().join("test.db");

        let url = db_path.to_str().expect("ok");
        let manager = ConnectionManager::<SqliteConnection>::new(url);

        let pool = r2d2::Pool::new(manager).unwrap();

        TestDb {
            tmp_dir,
            db_path: url.to_string(),
            pool,
        }
    }

    pub fn conn(&self) -> Option<PooledConnection<ConnectionManager<SqliteConnection>>> {
        let pool = self.pool.get().ok();
        let mut conn = pool.unwrap();
        let migrations = run_migrations(&mut conn);

        migrations.unwrap();
        self.pool.get().ok()
    }
}
