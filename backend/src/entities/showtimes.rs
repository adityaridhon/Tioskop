use chrono::{DateTime, Local};
use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "showtimes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub movie_id: Option<i64>,
    pub studio_id: Option<i64>,
    pub start_time: Option<DateTime<Local>>,
    pub price: Option<Decimal>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
