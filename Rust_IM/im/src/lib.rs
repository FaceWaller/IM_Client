mod error;
mod im_db;
mod im_manager;
mod im_model;
mod im_mqtt;
mod schema;
#[macro_use]
extern crate diesel;

pub use im_db::fetch_latest_msgs;
pub use im_manager::{add_recv, im_init, send_msg};
pub use im_model::{DBFetchIMModel, DBInsertIMModel};

uniffi::setup_scaffolding!();
