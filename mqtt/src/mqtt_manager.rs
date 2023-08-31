use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use rumqttc::*;
use once_cell::sync::OnceCell;
use super::error::*;

use std::thread;

pub(crate) static CLIENTMANAGER: OnceCell<Arc<Mutex<MqttManager>>> = OnceCell::new();
pub(crate) struct MqttManager {
    client: Client
}

// 连接
pub fn connect<F>(id: &str, host: &str, port: u16, notifi:F) -> MqttResult<()> 
where 
    F: Fn(usize, Result<Event, ConnectionError>) + Send + 'static
{
    let mut mqttoptions = MqttOptions::new(id, host, port);
    mqttoptions.set_keep_alive(Duration::from_secs(60));
    let (client, mut connection) = Client::new(mqttoptions, 10);

    thread::spawn(move ||  {
        for (i, notification) in connection.iter().enumerate() {
            notifi(i, notification);
        };
    });
   
    if let Some(lock_manager) = CLIENTMANAGER.get() {
        let mut mut_manager = lock_manager.lock().map_err(as_mqtt_error)?;
        mut_manager.client = client;
    } else {
        let manager = Mutex::new(MqttManager { client: client });
        CLIENTMANAGER.set( Arc::new(manager)).ok();
    }
    Ok(())
}
// 断开连接
pub fn disconnect() -> MqttResult<()> {
    if let Some(lock_manager) = CLIENTMANAGER.get() {
        let mut mut_manager = lock_manager.lock().map_err(as_mqtt_error)?;
        mut_manager.client.disconnect().map_err(as_mqtt_error)?;
    } else {
        let err = MqttError::CustomError("未连接".to_string());
        return Err(err);
    }
    Ok(())
}

// 订阅
pub fn subscribe(topic: String) -> MqttResult<()> {
    if let Some(lock_manager) = CLIENTMANAGER.get() {
        let mut mut_manager = lock_manager.lock().map_err(as_mqtt_error)?;
        mut_manager.client.subscribe(topic, QoS::AtLeastOnce).map_err(as_mqtt_error)?;
    } else {
        let err = MqttError::CustomError("未连接".to_string());
        return Err(err);
    }
    Ok(())
}

// 未订阅
pub fn unsubscribe(topic: String) -> MqttResult<()> {
    if let Some(lock_manager) = CLIENTMANAGER.get() {
        let mut mut_manager = lock_manager.lock().map_err(|err| MqttError::CustomError(err.to_string()))?;
        mut_manager.client.unsubscribe(topic).map_err(as_mqtt_error)?;
    } else {
        let err = MqttError::CustomError("未连接".to_string());
        return Err(err);
    }
    Ok(())
}

// publish
pub fn publish(topic: String, msg: String) -> MqttResult<()> {
    if let Some(lock_manager) = CLIENTMANAGER.get() {
        let mut mut_manager = lock_manager.lock().map_err(|err| MqttError::CustomError(err.to_string()))?;
        mut_manager.client.publish(topic, QoS::AtLeastOnce, true, msg.as_bytes().to_vec()).map_err(as_mqtt_error)?;
    } else {
        let err = MqttError::CustomError("未连接".to_string());
        return Err(err);
    }
    Ok(())
}

fn as_mqtt_error<E>(e: E) -> MqttError
where
    E: Debug + ToString
{
    MqttError::CustomError(e.to_string())
}