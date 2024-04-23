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
    id: u64,
    name: String,
    img: String,
    description: String,
    price: f64,

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
    name: String,
    description: String,
    price: f64,
    img: Option<String>,
}

#[derive(Fields, Default, Deserialize)]
pub struct GuitarraForUpdate {
    name: String,
    description: String,
    price: f64,
    img: Option<String>,
}

#[derive(Fields, Default, Deserialize)]
pub struct GuitarraForUpdateImg {
    img: String,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct GuitarraFilter {
    id: Option<OpValsInt64>,
    name: Option<OpValsString>,
    img: Option<OpValsString>,
    description: Option<OpValsString>,
    price: Option<OpValsFloat64>,

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_() -> Result<()> {
        todo!()
    }
}

// endregion:   --- Tests
