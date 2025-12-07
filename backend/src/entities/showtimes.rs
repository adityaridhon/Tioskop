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
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::movies::Entity",
        from = "Column::MovieId",
        to = "super::movies::Column::Id"
    )]
    Movies,
    #[sea_orm(
        belongs_to = "super::studios::Entity",
        from = "Column::StudioId",
        to = "super::studios::Column::Id"
    )]
    Studios,
    #[sea_orm(has_many = "super::bookings::Entity")]
    Bookings,
}

impl Related<super::movies::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Movies.def()
    }
}

impl Related<super::studios::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Studios.def()
    }
}

impl Related<super::bookings::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bookings.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
