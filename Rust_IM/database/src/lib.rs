use once_cell::sync::OnceCell;
mod error;
pub use error::*;
mod pool;
use pool::*;
pub mod schema;
pub use diesel_derives::*;
pub use diesel_migrations::*;
pub extern crate diesel;

static IMCONNPOOL: OnceCell<DBPool> = OnceCell::new();

pub fn init_db_path(db_path: &str) -> MyResult<()> {
    let pool = DBPool::new(db_path)?;
    IMCONNPOOL.get_or_init(|| pool);
    Ok(())
}

pub fn get_conn() -> MyResult<DBConn> {
    if let Some(pool) = IMCONNPOOL.get() {
        return pool.connect();
    };
    return Err(SQLiteError::CustomError("no init".to_string()));
}
