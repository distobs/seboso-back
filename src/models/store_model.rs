use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_postgres::{Row};

#[derive(Serialize)]
pub struct Store {
    pub id: i64,
    pub name: String,
    pub cnpj: String,
    pub street: String,
    pub number: i64,
    pub city: String,
    pub state: String,
    pub city_block: String,
    pub cep: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct CreateStore {
    pub name: String,
    pub cnpj: String,
    pub street: String,
    pub number: i64,
    pub city: String,
    pub state: String,
    pub city_block: String,
    pub cep: String,
}

#[derive(Deserialize)]
pub struct UpdateStore {
    pub name: Option<String>,
    pub cnpj: Option<String>,
    pub street: Option<String>,
    pub number: Option<i64>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub city_block: Option<String>,
    pub cep: Option<String>,
}

impl From<&Row> for Store {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            cnpj: row.get("cnpj"),
            street: row.get("street"),
            number: row.get("number"),
            city: row.get("city"),
            state: row.get("state"),
            city_block: row.get("city_block"),
            cep: row.get("cep"),
            created_at: row.get::<&str, DateTime<Utc>>("created_at").to_string(),
            updated_at: row.get::<&str, DateTime<Utc>>("updated_at").to_string(),
        }
    }
}
