pub mod mqtt_manager;
pub use mqtt_manager::*;

pub mod error;
pub use error::*;



// use std::{thread::{sleep, self}, time::Duration};
// use rumqttc::*;

// // use 
// fn main() {
//     println!("项目 start");

//     let mut mqttoptions = MqttOptions::new("111", "broker.emqx.io", 1883);
//     mqttoptions.set_keep_alive(Duration::from_secs(60));
//     let (mut client, mut connection) = Client::new(mqttoptions, 10);

    
//     client.subscribe("rumqtt", QoS::AtMostOnce).unwrap();

//     // thread::spawn(move || for i in 0..10 {
//     //    client.publish("rumqtt", QoS::AtLeastOnce, false, vec![i; i as usize]).unwrap();
//     //    thread::sleep(Duration::from_millis(100));
//     // });
    

//     for (i, notification) in connection.iter().enumerate() {
//         println!("Notification = {:?}", notification);    
//     };

//     sleep(Duration::from_secs(10));
//     println!("项目 End");
// }

