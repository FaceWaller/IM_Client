use super::error::*;
use super::im_db;
use super::im_mqtt;
use std::sync::{Mutex, Arc};
use once_cell::sync::OnceCell;

pub fn im_init(db_path: &str, id: &str, host: &str, port: u16, topic: &str) -> IMResult<()> {
    im_db::init_db(db_path)?;
    im_mqtt::im_connect(id, host, port)?;
    im_mqtt::im_subscribe(topic)?;
    Ok(())
}

pub(crate) static IMMANAGER: OnceCell<Arc<Mutex<IMManager>>> = OnceCell::new();

pub struct IMManager {

}

impl IMManager {
    
}
