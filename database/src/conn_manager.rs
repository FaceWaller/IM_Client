use super::error::*;
use super::database::*;
use super::conn_pool::*;
use std::{path::Path, sync::{Mutex, Arc}};
use diesel_migrations::*;
use once_cell::sync::OnceCell;

embed_migrations!("./migration/");  // 数据库升级
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
        std::fs::create_dir_all(storage_path).map_err(as_database_error)?;
    }
    let pool_config: PoolConfig = PoolConfig::default();
    let db_name = "im.sqlite";
    let database = Database::new(storage_path, db_name, pool_config).map_err(as_database_error)?;
    let conn = database.get_connection().map_err(as_database_error)?;
    let _ = setup_database(&*conn).map_err(as_database_error)?; // 创建版本表
    let _ = embedded_migrations::run(&*conn).map_err(as_database_error)?; // 升级数据库
    Ok(database)
}