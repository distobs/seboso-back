use axum::http::StatusCode;

use crate::{
        auth::jwt_auth::Claims, models::userstore_model::UserStore, types::{db_types::DbPool, response_types::ApiResponse}
};

pub async fn userstore_auth(
        pool: &DbPool,
        claims: Claims,
        user_id: i64,
        store_id: i64,
        required_role: &[&str],
) -> Result<bool, ApiResponse> {
        if claims.is_admin {
                return Ok(true);        
        }

        let rows = UserStore::from_user_store_id(pool, user_id, store_id)
        .await
        .map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;

        if let Some(row) = rows {
                if required_role.contains(&row.role.as_str()) {
                        return Ok(true);
                }
        }
        
        Ok(false)
}