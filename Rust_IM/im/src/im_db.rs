use super::error::*;
use super::im_model::*;
use database::diesel::*;
use database::diesel::{ExpressionMethods, RunQueryDsl};
use database::schema::{im_table, im_table::dsl};
use database::*;

pub(crate) fn init_db(db_path: &str) -> IMResult<()> {
    init_db_path(db_path).map_err(as_im_error)?;
    Ok(())
}

pub(crate) fn insert_msg(model: DBInsertIMModel) -> IMResult<()> {
    let conn: &mut diesel::SqliteConnection = &mut *get_conn().map_err(as_im_error)?;

    diesel::insert_into(im_table::table)
        .values(model.clone())
        .execute(conn)
        .map_err(as_im_error)?;
    Ok(())
}

pub fn delete_msg(im_sid: String) -> IMResult<()> {
    let conn: &mut diesel::SqliteConnection = &mut *get_conn().map_err(as_im_error)?;
    diesel::delete(dsl::im_table.filter(dsl::sid.eq(im_sid)))
        .execute(conn)
        .map_err(as_im_error)?;
    Ok(())
}

pub fn update_msg(model: DBChangestIMModel) -> IMResult<()> {
    let conn: &mut diesel::SqliteConnection = &mut *get_conn().map_err(as_im_error)?;
    let filter = dsl::im_table.filter(dsl::sid.eq(model.sid.clone()));
    diesel::update(filter)
        .set(model)
        .execute(conn)
        .map_err(as_im_error)?;
    Ok(())
}

pub fn fetch_latest_msgs(before_time: i64) -> IMResult<Vec<DBFetchIMModel>> {
    let conn: &mut diesel::SqliteConnection = &mut *get_conn().map_err(as_im_error)?;
    let query = dsl::im_table.filter(dsl::time.le(before_time));
    let res = query.load(conn).map_err(as_im_error)?;
    Ok(res)
}

pub fn fetch_latest_limit_msgs(before_time: i64, limit: i64) -> IMResult<Vec<DBFetchIMModel>> {
    let conn: &mut diesel::SqliteConnection = &mut *get_conn().map_err(as_im_error)?;
    let mut query = im_table::table.into_boxed();
    query = query.filter(dsl::time.le(before_time));
    query = query.limit(limit);
    let res = query.load(conn).map_err(as_im_error)?;
    Ok(res)
}
