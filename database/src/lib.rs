pub use diesel::*;
pub use diesel_derives::*;
pub use diesel_migrations::*;
pub use r2d2::PooledConnection;
pub use diesel::sql_types::Integer;
pub use std::{fmt::Debug, io, path::Path};
// pub mod conn_pool;
// pub use conn_pool::*;

// pub use migrations_internals::connection;
// #[doc(inline)]
// pub use migrations_internals::setup_database;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derives;
// #[allow(unused_imports)]
// #[macro_use]
// extern crate diesel_migrations;
