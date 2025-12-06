use sqlx::{mysql::MySqlPoolOptions, MySqlPool, Row};
use std::collections::HashMap;
use std::env;

pub struct DatabasePools {
    pub central: MySqlPool,
    pub cities: HashMap<String, MySqlPool>,
}

impl DatabasePools {
    pub async fn new() -> Self {
        // 1. Connect to Central DB first
        let central_url = env::var("DATABASE_URL_CENTRAL")
            .unwrap_or_else(|_| "mysql://root:@localhost:3306/tioskop_central_db".to_string());
        
        
        
        let central = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&central_url)
            .await
            .expect("❌ Failed to connect to Central DB");

        

        // 2. Read cities configuration from Central DB
        let cities_config = sqlx::query(
            "SELECT name, db_url FROM cities WHERE is_active = TRUE"
        )
        .fetch_all(&central)
        .await
        .expect("❌ Failed to fetch cities config from Central DB");

        // 3. Create pool for each city
        let mut cities = HashMap::new();
        for row in cities_config {
            let city_name: String = row.get("name");
            let db_url: String = row.get("db_url");
            
            
            
            let pool = MySqlPoolOptions::new()
                .max_connections(10)
                .connect(&db_url)
                .await
                .expect(&format!("❌ Failed to connect to {} DB", city_name));
            
            
            
            cities.insert(city_name.to_lowercase(), pool);
        }

        
        
        

        DatabasePools { central, cities }
    }

    pub fn get_city_pool(&self, city: &str) -> Option<&MySqlPool> {
        self.cities.get(&city.to_lowercase())
    }

    pub fn get_central(&self) -> &MySqlPool {
        &self.central
    }

    pub fn list_cities(&self) -> Vec<String> {
        self.cities.keys().cloned().collect()
    }
}

// Keep backward compatibility
pub async fn create_pool() -> MySqlPool {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://root:@localhost:3306/tioskop_central_db".to_string());
    
    MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database")
}
