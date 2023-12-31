use diesel::{dsl::sql, expression::SqlLiteral, query_dsl::LoadQuery, Connection, RunQueryDsl, SqliteConnection};
use super::error::{DataBaseResult, as_database_error};

pub trait ConnectionExtension: Connection {
    fn query<ST, T>(&self, query: &str) -> DataBaseResult<T>
    where
        SqlLiteral<ST>: LoadQuery<SqliteConnection, T>;

    fn exec(&self, query: impl AsRef<str>) -> DataBaseResult<usize>;
}

impl ConnectionExtension for SqliteConnection {
    fn query<ST, T>(&self, query: &str) -> DataBaseResult<T>
    where
        SqlLiteral<ST>: LoadQuery<SqliteConnection, T>,
    {
        Ok(sql::<ST>(query).get_result(self).map_err(as_database_error)?)
    }

    fn exec(&self, query: impl AsRef<str>) -> DataBaseResult<usize> {
        Ok(SqliteConnection::execute(self, query.as_ref()).map_err(as_database_error)?)
    }
}
