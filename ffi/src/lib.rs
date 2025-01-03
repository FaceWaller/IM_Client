use im::*;
use rand::Rng;
use std::time::SystemTime;
uniffi::setup_scaffolding!();

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum UniffiError {
    #[error("{0}")]
    IMError(String),
    #[error("sevice Not Found")]
    NotFound,
}

#[uniffi::export(callback_interface)]
pub trait MsgCall: Sync + Send {
    fn receive_msg(&self, record: DBInsertIMModel);
}

#[uniffi::export]
fn init_im(
    db_path: String,
    id: String,
    host: String,
    port: i32,
    recv_topic: String,
    call: Box<dyn MsgCall>,
) {
    // 回调函数
    let recv = move |msg: im::DBInsertIMModel| {
        call.receive_msg(msg);
    };

    // 调用 IM 库函数
    im::add_recv(recv).ok();
    im::im_init(&db_path, &id, &host, port as u16, &recv_topic).ok();
}

#[uniffi::export]
fn send_msg(from_id: String, to_id: String, send_topic: String, msg: String) {
    let sys_time = SystemTime::now();
    let new_sys_time = SystemTime::now();
    let difference = new_sys_time.duration_since(sys_time).unwrap().as_secs() as i64;
    let sid = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(30)
        .map(char::from)
        .collect::<String>();

    let msg = DBInsertIMModel {
        fromId: from_id,
        toId: to_id,
        time: difference,
        format: 0,
        sid: sid,
        msg: msg.to_string(),
    };
    im::send_msg(&send_topic, msg).ok();
}

#[uniffi::export]
fn delete_msg(im_sid: String) -> Result<(), UniffiError> {
    im::delete_msg(im_sid).map_err(|err| UniffiError::IMError(err.to_string()))?;
    Ok(())
}

#[uniffi::export]
fn update_msg(model: DBChangestIMModel) -> Result<(), UniffiError> {
    im::update_msg(model).map_err(|err| UniffiError::IMError(err.to_string()))?;
    Ok(())
}

#[uniffi::export]
fn fetch_latest_msgs(before_time: i64) -> Vec<DBFetchIMModel> {
    if let Ok(records) = im::fetch_latest_msgs(before_time) {
        records
    } else {
        vec![]
    }
}

#[uniffi::export]
fn fetch_latest_limit_msgs(before_time: i64, limit: i64) -> Vec<DBFetchIMModel> {
    if let Ok(records) = im::fetch_latest_limit_msgs(before_time, limit) {
        records
    } else {
        vec![]
    }
}
