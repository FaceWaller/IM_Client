use super::schema::im_table;
// use database::*;
use diesel::{
    AsChangeset, Associations, Identifiable, Insertable,
    Queryable,
};

#[allow(non_snake_case)]
#[derive(
    Default, PartialEq, Clone, Debug, Identifiable, Associations, Queryable,
)]
#[table_name = "im_table"]
#[primary_key(autoId)]

pub struct DBFetchIMModel {
    pub(crate) autoId: i64,
    pub(crate) formId: String,
    pub(crate) toId: String,
    pub(crate) time: i64,
    pub(crate) format: i32,
    pub(crate) sid: String,
    pub(crate) msg: String,
}

#[allow(non_snake_case)]
#[derive(
    Default, PartialEq, Clone, Debug, Associations, Insertable,
)]
#[table_name = "im_table"]
pub struct DBInsertIMModel {
    pub(crate) formId: String,
    pub(crate) toId: String,
    pub(crate) time: i64,
    pub(crate) format: i32,
    pub(crate) sid: String,
    pub(crate) msg: String,
}

#[allow(non_snake_case)]
#[derive(
    Default, PartialEq, Clone, Debug, Associations, AsChangeset,
)]
#[table_name = "im_table"]
pub struct DBChangestIMModel {
    pub(crate) formId: Option<String>,
    pub(crate) toId: Option<String>,
    pub(crate) time: Option<i64>,
    pub(crate) format: Option<i32>,
    pub(crate) sid: Option<String>,
    pub(crate) msg: Option<String>,
}