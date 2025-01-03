use crate::error::*;
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    RunQueryDsl, SqliteConnection,
};
use diesel_migrations::MigrationHarness;
use std::path::Path;

pub type DBConn = PooledConnection<ConnectionManager<SqliteConnection>>;
#[derive(Clone)]
pub struct DBPool {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl DBPool {
    pub fn new(path_str: &str) -> MyResult<DBPool> {
        let path: &Path = Path::new(path_str);
        let directory = path.parent().unwrap();
        std::fs::create_dir_all(directory).map_err(map_sqlite_error)?;

        let manager: ConnectionManager<SqliteConnection> =
            ConnectionManager::<SqliteConnection>::new(path.to_string_lossy());

        let pool = diesel::r2d2::Pool::builder()
            .connection_customizer(Box::new(ConnectionCustomizer {}))
            .build(manager)
            .map_err(map_sqlite_error)?;
        let db = DBPool { pool };
        db.migrate_up()?;
        Ok(db)
    }

    pub fn migrate_up(&self) -> MyResult<()> {
        let migrations = diesel_migrations::FileBasedMigrations::find_migrations_directory()
            .map_err(map_sqlite_error)?;
        let mut connection = self.connect()?;
        connection
            .run_pending_migrations(migrations)
            .and(Ok(()))
            .or(Err(map_sqlite_error("数据库已是最新")))
    }

    pub fn connect(&self) -> MyResult<DBConn> {
        self.pool.get().map_err(map_sqlite_error)
    }
}

#[derive(Debug)]
struct ConnectionCustomizer {}
impl diesel::r2d2::CustomizeConnection<SqliteConnection, diesel::r2d2::Error>
    for ConnectionCustomizer
{
    fn on_acquire(&self, connection: &mut SqliteConnection) -> Result<(), diesel::r2d2::Error> {
        let query = diesel::sql_query(
            r#"
			PRAGMA busy_timeout = 60000;
			PRAGMA journal_mode = WAL;
			PRAGMA synchronous = NORMAL;
			PRAGMA foreign_keys = ON;
		"#,
        );
        query
            .execute(connection)
            .map_err(diesel::r2d2::Error::QueryError)?;
        Ok(())
    }
}
