

fn main() {
    println!("IM启动");


    // sleep(Duration::from)
}


fn connect_mqtt() {
    let id = "im_9527";
    let host = "broker.emqx.io";
    let port = 1883;
    let topic = "rumqtt".to_string();

    
}


// use std::{str::from_utf8, path::Path};
// use rumqttc::{Event, Packet};
// use database::*;
// use std::env;

// fn main() {    
    
        // if let Some(current_path) = env::current_dir().unwrap().to_str() {
    //     let db_path = format!("{}/db", current_path);
       
    // }


    // let notify = |i: usize , notify: Result<Event, rumqttc::ConnectionError> | {
    //     if let Some(eve) = notify.ok() {
    //         match eve {
    //             Event::Incoming(data) => {
    //                 handle_packet(data);
    //             }
    //             Event::Outgoing(data) => {
    //                 println!("outgoing  {} {:?}", i , data);
    //             }
    //         }
    //     };
    // };
    // match mqtt::connect(id, host, port, notify) {
    //     Ok(_) => {
    //         println!("连接成功");
    //     }
    //     Err(e) => {
    //         println!("连接失败 {:?}", e);
    //     }
    // }
    // match mqtt::subscribe(topic) {
    //     Ok(_) => {
    //         println!("订阅成功");
    //     }
    //     Err(e) => {
    //         println!("订阅失败 {:?}", e);
    //     }
    // }
// }
