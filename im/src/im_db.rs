use database::*;
use super::error::*;
use super::im_model::*;

pub fn init_db(db_path: &str) -> IMResult<()> {
    init_connection(db_path).map_err(as_im_error)?;
    Ok(())
}

fn get_db_client() -> IMResult<DBConnection> {
    get_connection().map_err(as_im_error)
}

pub fn insert_model(model: DBInsertIMModel) {

}

pub fn delete_model(sid: String) {

}

pub fn update_model(model: DBChangestIMModel) {

}

pub fn fetch_model() {

}