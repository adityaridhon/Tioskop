use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "booking_seats")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub booking_id: Option<i64>,
    pub seat_id: Option<i64>,
    pub price: Option<Decimal>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bookings::Entity",
        from = "Column::BookingId",
        to = "super::bookings::Column::Id"
    )]
    Bookings,
    #[sea_orm(
        belongs_to = "super::seats::Entity",
        from = "Column::SeatId",
        to = "super::seats::Column::Id"
    )]
    Seats,
}

impl Related<super::bookings::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bookings.def()
    }
}

impl Related<super::seats::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Seats.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
