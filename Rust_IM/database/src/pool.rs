use crate::error::*;
use diesel::{
    connection::SimpleConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
    RunQueryDsl, SqliteConnection,
};
pub use diesel_migrations::embed_migrations;
pub use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::MigrationHarness;
use std::path::Path;

pub type DBConn = PooledConnection<ConnectionManager<SqliteConnection>>;
#[derive(Clone)]
pub struct DBPool {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

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

    fn migrate_up(&self) -> MyResult<()> {
        self.set_up_migration_db()?;
        let mut connection = self.connect()?;
        connection
            .run_pending_migrations(MIGRATIONS)
            .map_err(map_sqlite_error)?;
        Ok(())
    }

    fn set_up_migration_db(&self) -> MyResult<()> {
        let sql = format!(
            "    CREATE TABLE IF NOT EXISTS __diesel_schema_migrations (
            version VARCHAR(50) PRIMARY KEY NOT NULL,
            run_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        );"
        );
        let mut conn = self.connect()?;
        conn.batch_execute(&sql).map_err(map_sqlite_error)?;
        Ok(())
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
