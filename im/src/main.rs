use std::{thread::sleep, time::Duration, str::from_utf8};
use rumqttc::{Event, Packet};

fn main() {    
    let id = "im_9527";
    let host = "broker.emqx.io";
    let port = 1883;
    let topic = "rumqtt".to_string();

    let notify = |i: usize , notify: Result<Event, rumqttc::ConnectionError> | {
        if let Some(eve) = notify.ok() {
            match eve {
                Event::Incoming(data) => {
                    handle_packet(data);
                }
                Event::Outgoing(data) => {
                    println!("outgoing  {} {:?}", i , data);
                }
            }
        };
    };
    match mqtt::connect(id, host, port, notify) {
        Ok(_) => {
            println!("连接成功");
        }
        Err(e) => {
            println!("连接失败 {:?}", e);
        }
    }
    match mqtt::subscribe(topic) {
        Ok(_) => {
            println!("订阅成功");
        }
        Err(e) => {
            println!("订阅失败 {:?}", e);
        }
    }
    sleep(Duration::from_secs(100));
}

fn handle_packet(packet: Packet) {
    match packet {
        Packet::Connect(connect) => {
            println!("客户端发起连接 {:?}", connect);
        }
        Packet::ConnAck(conn_ack) => {
            println!("服务端确认连接 {:?}", conn_ack);
        }
        Packet::Publish(publish) => {
            println!("客户端/服务端 发布消息 {:?}", publish);

            if let Ok(utf8_str) = from_utf8(&publish.payload) {
                println!("xxxxx {}", utf8_str);
            }
        }
        Packet::PubAck(pub_ack) => {
            println!("客户端/服务端 确认发布消息 {:?}", pub_ack);
        }
        Packet::PubRec(pub_rec) => {
            println!("客户端/服务端 收到消息 {:?}", pub_rec);
        }
        Packet::PubRel(pub_rel) => {
            println!("客户端/服务端 消息释放 {:?}", pub_rel);
        }
        Packet::PubComp(pub_comp) => {
            println!("客户端/服务端 消息完成 {:?}", pub_comp);
        }
        Packet::Subscribe(subscribe) => {
            println!("客户端发起订阅 {:?}", subscribe);
        }
        Packet::SubAck(sub_ack) => {
            println!("服务端确认订阅 {:?}", sub_ack);
        }
        Packet::Unsubscribe(unsubscribe) => {
            println!("客户端取消订阅 {:?}", unsubscribe);
        }
        Packet:: UnsubAck(unsub_ack) => {
            println!("服务端确认取消订阅 {:?}", unsub_ack);
        }
        Packet::PingReq => {
            println!("客户端ping");
        }
        Packet::PingResp => {
            println!("服务端回复 ping");
        }
        Packet::Disconnect => {
            println!("客户端断开连接");
        }
    }
}

// PUBLISH（发布）： 发布者向服务器发送一条消息，并要求QoS级别2。
// PUBREC（发布收到确认）： 服务器接收到消息后，向发布者发送PUBREC消息，表示消息已经收到。
// PUBREL（发布释放）： 发布者收到PUBREC消息后，向服务器发送PUBREL消息，表示它已经准备好释放消息的发布状态。
// PUBCOMP（发布完成）： 服务器收到PUBREL消息后，向发布者发送PUBCOMP消息，表示消息发布已经完成，消息已成功传递。
    