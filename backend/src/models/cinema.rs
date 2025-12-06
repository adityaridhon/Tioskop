use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow, Clone, Debug)]
pub struct City {
    pub id: i64,
    pub name: String,
    pub db_name: String,
    pub db_url: String,
    pub is_active: bool,
}

#[derive(Serialize, Clone, Debug)]
pub struct CityInfo {
    pub id: i64,
    pub name: String,
    pub is_active: bool,
}

impl From<City> for CityInfo {
    fn from(city: City) -> Self {
        CityInfo {
            id: city.id,
            name: city.name,
            is_active: city.is_active,
        }
    }
}
