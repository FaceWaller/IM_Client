use super::error::*;
use super::im_db;
use super::im_model::DBInsertIMModel;
use super::im_mqtt;
use mqtt::mqtt_manager;
use once_cell::sync::OnceCell;
use std::sync::{Arc, Mutex};

pub fn im_init(db_path: &str, id: &str, host: &str, port: u16, topic: &str) -> IMResult<()> {
    im_db::init_db(db_path)?;
    im_mqtt::im_connect(id, host, port)?;
    im_mqtt::im_subscribe(topic)?;
    Ok(())
}

pub static IMMANAGER: OnceCell<Arc<Mutex<IMManager>>> = OnceCell::new();
pub struct IMManager {
    recv_msg: Option<Box<dyn Fn(DBInsertIMModel) + Send>>,
}

impl IMManager {
    pub(crate) fn receive_msg(&self, msg: DBInsertIMModel) -> IMResult<()> {
        im_db::insert_msg(msg.clone())?;
        if let Some(recv_msg) = &self.recv_msg {
            recv_msg(msg);
        }
        Ok(())
    }
}

pub fn add_recv<F>(recv: F) -> IMResult<()>
where
    F: Fn(DBInsertIMModel) + Send + 'static,
{
    let im_manager_ref =
        IMMANAGER.get_or_init(|| Arc::new(Mutex::new(IMManager { recv_msg: None })));

    let mut im_manager = im_manager_ref.lock().map_err(as_im_error)?;
    im_manager.recv_msg = Some(Box::new(recv));
    Ok(())
}

pub fn send_msg(topic: &str, msg: DBInsertIMModel) -> IMResult<()> {
    if let Some(manager) = IMMANAGER.get() {
        if let Some(im_manager) = manager.lock().ok() {
            im_manager.receive_msg(msg.clone()).ok();
        } else {
            println!("读取immanager异常");
        }
    }
    let msg_str = serde_json::to_string(&msg).map_err(as_im_error)?;
    mqtt_manager::publish(topic.to_string(), msg_str).map_err(as_im_error)?;
    Ok(())
}
