use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "bookings")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_id: Option<i64>,
    pub showtime_id: Option<i64>,
    pub booking_code: String,
    pub total_price: Option<Decimal>,
    pub payment_status: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id"
    )]
    Users,
    #[sea_orm(
        belongs_to = "super::showtimes::Entity",
        from = "Column::ShowtimeId",
        to = "super::showtimes::Column::Id"
    )]
    Showtimes,
    #[sea_orm(has_many = "super::booking_seats::Entity")]
    BookingSeats,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl Related<super::showtimes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Showtimes.def()
    }
}

impl Related<super::booking_seats::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BookingSeats.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
