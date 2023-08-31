// use std::{str::from_utf8, path::Path};
// use rumqttc::{Event, Packet};
use database::*;
// use std::env;
use super::error::*;

// pub fn init_db(db_path: &str) -> IMResult<()> {
//     // if let Some(current_path) = env::current_dir().unwrap().to_str() {
//     //     let db_path = format!("{}/db", current_path);
       
//     // }

//     get_connection(db_path).map_err(as_im)
//     match get_connection(db_path) {
//         Ok(_) => {
//             println!("数据库连接成功 {:?}", db_path);
//             Ok(())
//         }
//         Err(e) => {
//             println!("数据库连接失败 {}",e);
//             Err(IMError::CustomError(e.to_string()))
//         }
//     }
// }
