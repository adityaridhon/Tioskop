use chrono::NaiveDate;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "movies")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub title: String,
    pub genre: Option<String>,
    pub rating: Option<String>,
    pub duration: Option<i32>,
    pub description: Option<String>,
    pub poster_url: Option<String>,
    pub release_date: Option<NaiveDate>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::showtimes::Entity")]
    Showtimes,
}

impl Related<super::showtimes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Showtimes.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
