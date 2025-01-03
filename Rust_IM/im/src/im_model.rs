use database::diesel;
// use database::diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use database::diesel::*;
use database::schema::*;
use serde;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(uniffi::Record, Serialize, Deserialize, Default, PartialEq, Clone, Debug, Queryable)]
#[diesel(table_name = im_table)]
pub struct DBFetchIMModel {
    pub(crate) autoId: i64,
    pub(crate) fromId: String,
    pub(crate) toId: String,
    pub(crate) time: i64,
    pub(crate) format: i32,
    pub(crate) sid: String,
    pub(crate) msg: String,
}

#[allow(non_snake_case)]
#[derive(uniffi::Record, Serialize, Deserialize, Default, PartialEq, Clone, Debug, Insertable)]
#[diesel(table_name = im_table)]
pub struct DBInsertIMModel {
    pub fromId: String,
    pub toId: String,
    pub time: i64,
    pub format: i32,
    pub sid: String,
    pub msg: String,
}

impl DBInsertIMModel {
    pub fn to_json_string(&self) -> String {
        serde_json::to_string(&self).expect("Failed to serialize to JSON")
    }
}
#[allow(non_snake_case)]
#[derive(uniffi::Record, Serialize, Deserialize, Default, PartialEq, Clone, Debug, AsChangeset)]
#[table_name = "im_table"]
pub struct DBChangestIMModel {
    pub(crate) fromId: Option<String>,
    pub(crate) toId: Option<String>,
    pub(crate) time: Option<i64>,
    pub(crate) format: Option<i32>,
    pub(crate) sid: String,
    pub(crate) msg: Option<String>,
}
