use std::{thread::{sleep, self}, time::{Duration, SystemTime}, env};
use im::{self, DBInsertIMModel};
use rand::Rng;

fn main() {
    println!("IM启动");
    let db_path;

    if let Some(current_path) = env::current_dir().unwrap().to_str() {
        db_path = format!("{}/db", current_path);
    } else {
        println!("没能获取当前位置");
        return;
    }

    // 网络配置，这里使用了 mqtt 免费服务器
    let id = "im_9528";
    let host = "broker.emqx.io";
    let port = 1883;
    let recv_topic = "rumqtt_9528";

    let recv = |msg:im::DBInsertIMModel| {
        println!("客户端 B 收到消息 {:?}", msg);
    };
    im::add_recv(recv).ok();
    im::im_init(db_path.as_str(), id, host, port, recv_topic).ok();
    
    let send_topic = "rumqtt_9527";
    // 隔 5 秒发一条消息
    thread::spawn(|| {
        loop {
            let sys_time = SystemTime::now();
            let new_sys_time = SystemTime::now();
            let difference = new_sys_time.duration_since(sys_time).unwrap().as_secs() as i64;
            let sid = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(30)
            .map(char::from)
            .collect::<String>();

            let msg = DBInsertIMModel {
                fromId: "9528".to_string(),
                toId: "9527".to_string(),
                time: difference,
                format: 0,
                sid: sid,
                msg: "B 发送给 A 的文本消息".to_string()
            };
            im::send_msg(&send_topic.to_string(), msg).ok();
            thread::sleep(Duration::from_secs(5));
        }
    });

    sleep(Duration::from_secs(100)); // 避免线程退出
}