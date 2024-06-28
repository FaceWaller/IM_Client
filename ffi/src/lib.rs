
use libc::c_char;
use std::{ffi::CStr, ffi::CString};
use im::{self, DBInsertIMModel};
use std::time::SystemTime;
use rand::Rng;

type RecvCallback = Option<unsafe extern "C" fn(*const c_char)>;

#[no_mangle]
pub extern "C" fn init_im(db_path: *const c_char, id: *const c_char, host: *const c_char, port: i32, recv_topic: *const c_char, recv_callback: RecvCallback) {
    // 安全地将 C 字符串转换为 Rust 字符串
    let db_path = unsafe {
        assert!(!db_path.is_null());
        CStr::from_ptr(db_path).to_str().expect("Invalid UTF-8")
    };
    let id = unsafe {
        assert!(!id.is_null());
        CStr::from_ptr(id).to_str().expect("Invalid UTF-8")
    };
    let host = unsafe {
        assert!(!host.is_null());
        CStr::from_ptr(host).to_str().expect("Invalid UTF-8")
    };
    let recv_topic = unsafe {
        assert!(!recv_topic.is_null());
        CStr::from_ptr(recv_topic).to_str().expect("Invalid UTF-8")
    };
    let recv_callback_cloned = recv_callback;

    // 回调函数
    let recv = move |msg: im::DBInsertIMModel| {
       let msg_str = msg.to_json_string();

        if let Some(callback) = recv_callback_cloned {
            let c_msg_str: CString = CString::new(msg_str).expect("CString::new failed");
            unsafe {
                callback(c_msg_str.as_ptr());
            }
        }
    };

    // 调用 IM 库函数
    im::add_recv(recv).ok();
    im::im_init(db_path, id, host, port as u16, recv_topic).ok();
    println!("xxxxxx初始化IM");
}

#[no_mangle]
pub extern "C" fn send_msg(from_id:*const c_char, to_id: *const c_char, send_topic: *const c_char, msg: *const c_char) {
    let sys_time = SystemTime::now();
    let new_sys_time = SystemTime::now();
    let difference = new_sys_time.duration_since(sys_time).unwrap().as_secs() as i64;
    let sid = rand::thread_rng()
    .sample_iter(&rand::distributions::Alphanumeric)
    .take(30)
    .map(char::from)
    .collect::<String>();


    let from_id: &str = unsafe {
        assert!(!from_id.is_null());
        CStr::from_ptr(from_id).to_str().expect("Invalid UTF-8")
    };
    let to_id: &str = unsafe {
        assert!(!to_id.is_null());
        CStr::from_ptr(to_id).to_str().expect("Invalid UTF-8")
    };
    let send_topic = unsafe {
        assert!(!send_topic.is_null());
        CStr::from_ptr(send_topic).to_str().expect("Invalid UTF-8")
    };
    let msg: &str = unsafe {
        assert!(!msg.is_null());
        CStr::from_ptr(msg).to_str().expect("Invalid UTF-8")
    };

    let msg = DBInsertIMModel {
        fromId: from_id.to_string(),
        toId: to_id.to_string(),
        time: difference,
        format: 0,
        sid: sid,
        msg: msg.to_string()
    };
    im::send_msg(&send_topic.to_string(), msg).ok();
}