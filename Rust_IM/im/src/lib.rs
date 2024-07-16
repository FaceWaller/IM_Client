mod error;
mod im_db;
mod im_mqtt;
mod im_model;
mod im_manager;
mod schema;
#[macro_use]
extern crate diesel;

pub use im_model::DBInsertIMModel;
pub use im_manager::{im_init, add_recv, send_msg};
pub use im_db::fetch_latest_msgs;

