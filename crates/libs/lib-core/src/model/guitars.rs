// region:      --- Modules

use crate::ctx::Ctx;
use crate::generate_common_bmc_fns;
use crate::model::base::{self, DbBmc};
use crate::model::modql_utils::time_to_sea_value;
use crate::model::ModelManager;
use crate::model::Result;
use lib_utils::time::Rfc3339;
use modql::field::Fields;
use modql::filter::{
    FilterNodes, ListOptions, OpValsBool, OpValsFloat64, OpValsInt64, OpValsString, OpValsValue,
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;

// endregion:   --- Modules

// region:      --- Guitar types

#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Guitarra {
    pub id: u64,
    pub name: String,
    pub img: String,
    pub description: String,
    pub price: f64,

    // --   Timestamps
    //      (Creator and last modifier)
    pub cid: i64,
    #[serde_as(as = "Rfc3339")]
    pub ctime: OffsetDateTime,
    pub mid: i64,
    #[serde_as(as = "Rfc3339")]
    pub mtime: OffsetDateTime,
}

#[derive(Fields, Deserialize)]
pub struct GuitarraForCreate {
    pub name: String,
    pub description: String,
    pub price: f64,
    pub img: Option<String>,
}

#[derive(Fields, Default, Deserialize)]
pub struct GuitarraForUpdate {
    pub name: String,
    pub description: String,
    pub price: f64,
    pub img: Option<String>,
}

#[derive(Fields, Default, Deserialize)]
pub struct GuitarraForUpdateImg {
    pub img: String,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct GuitarraFilter {
    pub id: Option<OpValsInt64>,
    pub name: Option<OpValsString>,
    pub img: Option<OpValsString>,
    pub description: Option<OpValsString>,
    pub price: Option<OpValsFloat64>,

    // --   Timestamps
    //      (Creator and last modifier)
    pub cid: i64,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    pub ctime: Option<OpValsValue>,
    pub mid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    pub mtime: Option<OpValsValue>,
}

// endregion:   --- Guitar types

// region:      --- TaskBmc

pub struct GuitarraBmc;

impl DbBmc for GuitarraBmc {
    const TABLE: &'static str = "guitarras";
}

generate_common_bmc_fns!(
    Bmc: GuitarraBmc,
    Entity: Guitarra,
    ForCreate: GuitarraForCreate,
    ForUpdate: GuitarraForUpdate,
    Filter: GuitarraFilter,
);

// endregion:   --- TaskBmc

// region:      --- Tests

// TODO: Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::_dev_utils;
    use crate::model::Error;
    use anyhow::Result;
    use lib_utils::time::{format_time, now_utc};
    use modql::filter::OpValString;
    use serde_json::json;
    use serial_test::serial;
    use std::time::Duration;
    use tokio::time::sleep;

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        todo!()
    }

    #[serial]
    #[tokio::test]
    async fn test_get_err_not_found() -> Result<()> {
        todo!()
    }

    #[serial]
    #[tokio::test]
    async fn test_list_all_ok() -> Result<()> {
        todo!()
    }

    #[serial]
    #[tokio::test]
    async fn test_list_by_guitar_contains_ok() -> Result<()> {
        todo!()
    }

    #[serial]
    #[tokio::test]
    async fn test_list_with_list_options_ok() -> Result<()> {
        todo!()
    }

    #[serial]
    #[tokio::test]
    async fn test_list_by_ctime_ok() -> Result<()> {
        todo!()
    }

    #[serial]
    #[tokio::test]
    async fn test_update_ok() -> Result<()> {
        todo!()
    }

    #[serial]
    #[tokio::test]
    async fn test_delete_err_not_found() -> Result<()> {
        todo!()
    }
}

// endregion:   --- Tests
