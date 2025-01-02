use super::error::*;
use super::im_model::*;
use super::schema::{im_table, im_table::dsl};
use database::*;
use diesel::prelude::*;

pub(crate) fn init_db(db_path: &str) -> IMResult<()> {
    init_connection(db_path).map_err(as_im_error)?;
    Ok(())
}

fn get_db_client() -> IMResult<DBConnection> {
    get_connection().map_err(as_im_error)
}

pub(crate) fn insert_msg(model: DBInsertIMModel) -> IMResult<()> {
    diesel::insert_into(im_table::table)
        .values(model.clone())
        .execute(&*get_db_client()?)
        .map_err(as_im_error)?;
    Ok(())
}

#[allow(dead_code)]
pub(crate) fn delete_msg(im_sid: String) -> IMResult<()> {
    diesel::delete(dsl::im_table.filter(dsl::sid.eq(im_sid)))
        .execute(&*get_db_client()?)
        .map_err(as_im_error)?;
    Ok(())
}

#[allow(dead_code)]
pub(crate) fn update_msg(model: DBChangestIMModel) -> IMResult<()> {
    let filter = dsl::im_table.filter(dsl::sid.eq(model.sid.clone()));
    diesel::update(filter)
        .set(model)
        .execute(&*get_db_client()?)
        .map_err(as_im_error)?;
    Ok(())
}

#[allow(dead_code)]
pub fn fetch_latest_msgs(before_time: i64) -> IMResult<Vec<DBFetchIMModel>> {
    let query = dsl::im_table.into_boxed().filter(dsl::time.le(before_time));
    let res = query.load(&*get_db_client()?).map_err(as_im_error)?;
    Ok(res)
}

#[allow(dead_code)]
pub fn fetch_latest_limit_msgs(before_time: i64, limit: i64) -> IMResult<Vec<DBFetchIMModel>> {
    let query = dsl::im_table
        .into_boxed()
        .filter(dsl::time.le(before_time))
        .limit(limit);
    let res = query.load(&*get_db_client()?).map_err(as_im_error)?;
    Ok(res)
}
