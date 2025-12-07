use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "studios")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub cinema_id: Option<i64>,
    pub name: String,
    pub capacity: i32,
    #[sea_orm(column_name = "type")]
    pub r#type: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::showtimes::Entity")]
    Showtimes,
    #[sea_orm(has_many = "super::seats::Entity")]
    Seats,
}

impl Related<super::showtimes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Showtimes.def()
    }
}

impl Related<super::seats::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Seats.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
