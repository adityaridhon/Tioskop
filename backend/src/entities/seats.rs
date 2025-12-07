use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "seats")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub studio_id: Option<i64>,
    pub seat_code: String,
    pub seat_row: Option<i32>,
    pub seat_col: Option<i32>,
    pub seat_status: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::studios::Entity",
        from = "Column::StudioId",
        to = "super::studios::Column::Id"
    )]
    Studios,
    #[sea_orm(has_many = "super::booking_seats::Entity")]
    BookingSeats,
}

impl Related<super::studios::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Studios.def()
    }
}

impl Related<super::booking_seats::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BookingSeats.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
