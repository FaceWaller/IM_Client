use super::schema::im_table;
use serde;
use serde::{Deserialize, Serialize};

use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable};

#[allow(non_snake_case)]
#[derive(Serialize, Default, PartialEq, Clone, Debug, Identifiable, Associations, Queryable)]
#[table_name = "im_table"]
#[primary_key(autoId)]

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
#[derive(
    uniffi::Record,
    Serialize,
    Deserialize,
    Default,
    PartialEq,
    Clone,
    Debug,
    Associations,
    Insertable,
)]
#[table_name = "im_table"]
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
#[derive(Default, PartialEq, Clone, Debug, Associations, AsChangeset)]
#[table_name = "im_table"]
pub struct DBChangestIMModel {
    pub(crate) fromId: Option<String>,
    pub(crate) toId: Option<String>,
    pub(crate) time: Option<i64>,
    pub(crate) format: Option<i32>,
    pub(crate) sid: String,
    pub(crate) msg: Option<String>,
}
