#![allow(clippy::upper_case_acronyms)]
use diesel::{
    expression::SqlLiteral,
    query_dsl::load_dsl::LoadQuery,
    sql_types::{Integer, Text},
    SqliteConnection,
};
use super::error::*;
use super::conn_ext::ConnectionExtension;
use std::{
    convert::{TryFrom, TryInto},
    fmt,
    str::FromStr,
};

pub trait PragmaExtension: ConnectionExtension {
    fn pragma<D: std::fmt::Display>(&self, key: &str, val: D, schema: Option<&str>) -> DataBaseResult<()> {
        let query = match schema {
            Some(schema) => format!("PRAGMA {}.{} = '{}'", schema, key, val),
            None => format!("PRAGMA {} = '{}'", key, val),
        };
        self.exec(&query).map_err(|err| as_database_error(err.to_string()))?;
        Ok(())
    }

    fn pragma_ret<ST, T, D: std::fmt::Display>(&self, key: &str, val: D, schema: Option<&str>) -> DataBaseResult<T>
    where
        SqlLiteral<ST>: LoadQuery<SqliteConnection, T>,
    {
        let query = match schema {
            Some(schema) => format!("PRAGMA {}.{} = '{}'", schema, key, val),
            None => format!("PRAGMA {} = '{}'", key, val),
        };
        self.query::<ST, T>(&query)
    }

    fn pragma_get<ST, T>(&self, key: &str, schema: Option<&str>) -> DataBaseResult<T>
    where
        SqlLiteral<ST>: LoadQuery<SqliteConnection, T>,
    {
        let query = match schema {
            Some(schema) => format!("PRAGMA {}.{}", schema, key),
            None => format!("PRAGMA {}", key),
        };
        self.query::<ST, T>(&query)
    }

    fn pragma_set_busy_timeout(&self, timeout_ms: i32) -> DataBaseResult<i32> {
        self.pragma_ret::<Integer, i32, i32>("busy_timeout", timeout_ms, None)
    }

    fn pragma_get_busy_timeout(&self) -> DataBaseResult<i32> {
        self.pragma_get::<Integer, i32>("busy_timeout", None)
    }

    fn pragma_set_journal_mode(&self, mode: SQLiteJournalMode, schema: Option<&str>) -> DataBaseResult<i32> {
        self.pragma_ret::<Integer, i32, SQLiteJournalMode>("journal_mode", mode, schema)
    }

    fn pragma_get_journal_mode(&self, schema: Option<&str>) -> DataBaseResult<SQLiteJournalMode> {
        self.pragma_get::<Text, String>("journal_mode", schema)?.parse()
    }

    fn pragma_set_synchronous(&self, synchronous: SQLiteSynchronous, schema: Option<&str>) -> DataBaseResult<()> {
        self.pragma("synchronous", synchronous as u8, schema)
    }

    fn pragma_get_synchronous(&self, schema: Option<&str>) -> DataBaseResult<SQLiteSynchronous> {
        self.pragma_get::<Integer, i32>("synchronous", schema)?.try_into()
    }
}
impl PragmaExtension for SqliteConnection {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SQLiteJournalMode {
    DELETE,
    TRUNCATE,
    PERSIST,
    MEMORY,
    WAL,
    OFF,
}

impl fmt::Display for SQLiteJournalMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::DELETE => "DELETE",
                Self::TRUNCATE => "TRUNCATE",
                Self::PERSIST => "PERSIST",
                Self::MEMORY => "MEMORY",
                Self::WAL => "WAL",
                Self::OFF => "OFF",
            }
        )
    }
}

impl FromStr for SQLiteJournalMode {
    type Err = DataBaseError;
    fn from_str(s: &str) -> DataBaseResult<Self> {
        match s.to_uppercase().as_ref() {
            "DELETE" => Ok(Self::DELETE),
            "TRUNCATE" => Ok(Self::TRUNCATE),
            "PERSIST" => Ok(Self::PERSIST),
            "MEMORY" => Ok(Self::MEMORY),
            "WAL" => Ok(Self::WAL),
            "OFF" => Ok(Self::OFF),
            _ => Err(DataBaseError::CustomError("err".to_string())),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SQLiteSynchronous {
    EXTRA = 3,
    FULL = 2,
    NORMAL = 1,
    OFF = 0,
}

impl fmt::Display for SQLiteSynchronous {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::OFF => "OFF",
                Self::NORMAL => "NORMAL",
                Self::FULL => "FULL",
                Self::EXTRA => "EXTRA",
            }
        )
    }
}

impl TryFrom<i32> for SQLiteSynchronous {
    type Error = DataBaseError;

    fn try_from(v: i32) -> DataBaseResult<Self> {
        match v {
            0 => Ok(Self::OFF),
            1 => Ok(Self::NORMAL),
            2 => Ok(Self::FULL),
            3 => Ok(Self::EXTRA),
            _ => Err(DataBaseError::CustomError("err".to_string())),
        }
    }
}


impl FromStr for SQLiteSynchronous {
    type Err = DataBaseError;
    fn from_str(s: &str) -> DataBaseResult<Self> {
        match s.to_uppercase().as_ref() {
            "0" | "OFF" => Ok(Self::OFF),
            "1" | "NORMAL" => Ok(Self::NORMAL),
            "2" | "FULL" => Ok(Self::FULL),
            "3" | "EXTRA" => Ok(Self::EXTRA),
            _ => Err(DataBaseError::CustomError("err".to_string())),
        }
    }
}
