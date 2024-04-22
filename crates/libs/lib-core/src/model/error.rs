use derive_more::From;
use lib_auth::pwd;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use sqlx::error::DatabaseError;
use std::borrow::Cow;
use crate::model::store;

// region:     Error

pub type Result<T> = std::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From)]
pub enum Error {
    EntityNotFound {
        entity: &'static str,
        id: i64,
    },
    ListLimitOverMax {
        max: i64,
        actual: i64,
    },

    // -- Modules
    #[from]
    Pwd(pwd::Error),
    #[from]
    Store(store::Error),

    // -- Db
    UserAlreadyExists {
        username: String,
    },
    UniqueViolation {
        table: String,
        constraint: String,
    },

    // -- ModelManager
    CantCreateModelManagerProvider(String),

    // -- Externals
    #[from]
    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
    #[from]
    SeaQuery(#[serde_as(as = "DisplayFromStr")] sea_query::error::Error),
    #[from]
    ModqlIntoSea(#[serde_as(as = "DisplayFromStr")] modql::filter::IntoSeaError),
}

// region:     Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for Error {}
// endregion:  Error Boilerplate

// endregion:  Error
