use std::str::from_utf8;
use rumqttc::{Event, Packet};
use mqtt;
use crate::{error::{IMResult, as_im_error}, DBInsertIMModel, im_manager::IMMANAGER};

pub fn im_connect(id: &str, host: &str, port: u16) -> IMResult<()> 
{
    let notifi = |i: usize , notify: Result<Event, rumqttc::ConnectionError> | {
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
    mqtt::connect(id, host, port, notifi).map_err(as_im_error)?;
    Ok(())
}

pub fn im_subscribe(topic:&str) -> IMResult<()> {
    mqtt::subscribe(topic.to_string()).map_err(as_im_error)
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
                let msg_res: Result<DBInsertIMModel, _> = serde_json::from_str(utf8_str);
                match msg_res {
                    Ok(msg) => {
                        if let Some(manager) = IMMANAGER.get() {
                            if let Some(im_manager) = manager.lock().ok() {
                                im_manager.receive_msg(msg).ok();
                            } else {
                                println!("读取immanager异常");
                            }
                        } else {
                            println!("IMMANAGER 未初始化");
                        }
                    }
                    Err(e) => {
                        println!("消息格式有误 {:?}", e);
                    }
                }


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
    