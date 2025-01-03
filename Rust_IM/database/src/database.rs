use super::conn_pool::*;
use super::error::*;
use r2d2::PooledConnection;
use std::sync::Arc;

pub struct Database {
    pool: Arc<ConnectionPool>,
}

pub type DBConnection = PooledConnection<ConnectionManager>;

impl Database {
    pub fn new(dir: &str, name: &str, pool_config: PoolConfig) -> DataBaseResult<Self> {
        let uri = db_file_uri(dir, name);
        let pool = ConnectionPool::new(pool_config, &uri)?;
        Ok(Self {
            pool: Arc::new(pool),
        })
    }

    pub fn get_connection(&self) -> DataBaseResult<DBConnection> {
        let conn = self.pool.get().map_err(as_database_error)?;
        Ok(conn)
    }
}

pub fn db_file_uri(dir: &str, name: &str) -> String {
    use std::path::MAIN_SEPARATOR;
    let mut uri = dir.to_owned();
    if !uri.ends_with(MAIN_SEPARATOR) {
        uri.push(MAIN_SEPARATOR);
    }
    uri.push_str(name);
    uri
}
