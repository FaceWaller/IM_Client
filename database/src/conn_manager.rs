use diesel_migrations::*;
use super::database::*;
use super::conn_pool::*;
#[allow(unused_imports)]
use r2d2::PooledConnection;
use std::{path::Path, sync::{Mutex, Arc}};
use super::error::*;
pub mod prelude {
    pub use crate::*;
    pub use diesel::SqliteConnection;
    pub use diesel::{query_dsl::*, BelongingToDsl, ExpressionMethods, RunQueryDsl};
}
use once_cell::sync::OnceCell;

embed_migrations!("./migration/");

pub(crate) static IMDATABASE: OnceCell<Arc<Mutex<Database>>> = OnceCell::new();

pub fn get_connection(storage_path: &str) -> DataBaseResult<DBConnection, DataBaseError> {
    if let Some(lock_conn) = IMDATABASE.get() {
        let database = lock_conn.lock().map_err(as_database_error)?;
        let conn = database.get_connection().map_err(as_database_error)?;
        Ok(conn)
    } else {
        let database = init(storage_path)?;
        let conn = database.get_connection().map_err(as_database_error)?;
        IMDATABASE.set(Arc::new(Mutex::new(database))).ok();
        Ok(conn)
    }
}

fn init(storage_path: &str) -> DataBaseResult<Database> {
    if !Path::new(storage_path).exists() {
        std::fs::create_dir_all(storage_path).map_err(|err| as_database_error(err.to_string()))?;
    }
    let pool_config: PoolConfig = PoolConfig::default();
    let db_name = "im.sqlite";
    let database = Database::new(storage_path, db_name, pool_config).map_err(|err| as_database_error(err.to_string()))?;
    let conn = database.get_connection().map_err(|err| as_database_error(err.to_string()))?;
    let _ = setup_database(&*conn).map_err(|err| as_database_error(err.to_string()))?; // 创建版本表
    let _ = embedded_migrations::run(&*conn); // 升级数据库
    Ok(database)
}