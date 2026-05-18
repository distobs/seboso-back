use serde::{Deserialize, Serialize};
use tokio_postgres::{Row};

#[derive(Serialize, Deserialize)]
pub struct Catalog {
    pub store_id: i64,
    pub isbn_10_code_book: i64,
    pub price: f32,
    pub quantity: i64,
    pub description: String,
}

#[derive(Deserialize)]
pub struct UpdateCatalog {
    pub isbn_10_code_book: i64,
    pub price: Option<f32>,
    pub quantity: Option<i64>,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct CatalogQuery {
    pub store_id: i64,
    pub isbn_10_code_book: i64,
}

impl From<&Row> for Catalog {
    fn from(row: &Row) -> Self {
        Self {
            store_id: row.get("store_id"),
            isbn_10_code_book: row.get("isbn_10_code_book"),
            price: row.get("price"),
            quantity: row.get("quantity"),
            description: row.get("store_id"),
        }
    }
}
