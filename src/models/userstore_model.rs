use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use crate::{types::db_types::DbPool, utils::error_utils::DynError};

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct UserStore {
    pub store_id: i64,
    pub user_id: i64,
    pub role: String,
}

#[derive(Deserialize)]
pub struct CreateUserStore {
    pub user_id: i64,
    pub role: String,
}

impl From<&Row> for UserStore {
    fn from(row: &Row) -> Self {
        Self {
            store_id: row.get("store_id"),
            user_id: row.get("user_id"),
            role: row.get("role"),
        }
    }
}

impl UserStore {
    pub async fn from_user_id(pool: &DbPool, user_id: i64) -> Result<Vec<UserStore>, DynError> {
        let conn = pool.get().await?;

        let rows = conn
            .query("SELECT * FROM user_store WHERE user_id = $1", &[&user_id])
            .await?;

        Ok(rows.iter().map(UserStore::from).collect())
    }

    pub async fn from_store_id(
        pool: &DbPool,
        store_id: i64,
    ) -> Result<Vec<UserStore>, DynError> {
        let conn = pool.get().await?;

        let rows = conn
            .query("SELECT * FROM user_store WHERE store_id = $1", &[&store_id])
            .await?;

        Ok(rows.iter().map(UserStore::from).collect())
    }
    
    pub async fn from_user_store_id(
        pool: &DbPool,
        user_id: i64,
        store_id: i64,
    ) -> Result<Option<UserStore>, DynError> {
        let conn = pool.get().await?;

        let row = conn
            .query_opt(
                "SELECT * FROM user_store WHERE user_id = $1 AND store_id = $2",
                &[&user_id, &store_id],
            )
            .await?;

        match row {
            Some(row) => Ok(Some(UserStore::from(&row))),
            None => Ok(None),
        }
    }
}
