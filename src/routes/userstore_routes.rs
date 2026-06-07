use axum::{
        Extension, Json, Router, extract::{self, Query, State}, http::StatusCode, middleware, routing::{get, post}
};

use crate::{auth::{jwt_auth::{Claims, jwt_middleware}, userstore_auth::userstore_auth}, models::userstore_model::{QueryUserStore, UserStore}, types::{db_types::DbPool, response_types::ApiResponse}};

async fn create_userstore(
        State(pool): State<DbPool>,
        Extension(claims): Extension<Claims>,
        payload: extract::Json<UserStore>,
) -> Result<ApiResponse, ApiResponse> {
        let validate = userstore_auth(
                &pool,
                claims,
                payload.store_id,
                &["owner"],
        ).await.map_err(
                |_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR)
        )?;

        if !validate {
                return Err(ApiResponse::err(StatusCode::FORBIDDEN));
        }

        let conn = pool.get().await.map_err(
                |_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR)
        )?;
        
        conn.query(
                "INSERT INTO user_store (user_id, store_id, role) VALUES ($1, $2, $3)",
                &[&payload.user_id, &payload.store_id, &payload.role]
        ).await.map_err(
                |_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR
        ))?;

        Ok(ApiResponse::ok_msg("Relação criada com sucesso."))
}

async fn update_userstore(
        State(pool): State<DbPool>,
        Extension(claims): Extension<Claims>,
        Json(payload): Json<UserStore>,
) -> Result<ApiResponse, ApiResponse> {
        let validate = userstore_auth(
                &pool,
                claims,
                payload.store_id,
                &["owner"],
        ).await.map_err(
                |_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR)
        )?;

        if !validate {
                return Err(ApiResponse::err(StatusCode::FORBIDDEN));
        }

        let conn = pool.get().await.map_err(
                |_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR)
        )?;

        conn.query(
                "UPDATE user_store SET role = $1 WHERE user_id = $2 AND store_id = $3",
                &[&payload.role, &payload.user_id, &payload.store_id]
        ).await.map_err(
                |_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR)
        )?;

        Ok(ApiResponse::ok_msg("Relação atualizada com sucesso."))
}

async fn delete_userstore(
        State(pool): State<DbPool>,
        Extension(claims): Extension<Claims>,
        Json(payload): Json<UserStore>,
) -> Result<ApiResponse, ApiResponse> {
        let validate = userstore_auth(
                &pool,
                claims,
                payload.store_id,
                &["owner"],
        ).await.map_err(
                |_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR)
        )?;

        if !validate {
                return Err(ApiResponse::err(StatusCode::FORBIDDEN));
        }

        let conn = pool.get().await.map_err(
                |_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR)
        )?;

        conn.query(
                "DELETE FROM user_store WHERE user_id = $1 AND store_id = $2 AND role = $3",
                &[&payload.user_id, &payload.store_id, &payload.role]
        ).await.map_err(
                |_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR)
        )?;

        Ok(ApiResponse::ok_msg("Relação deletada com sucesso."))
}

async fn get_userstore(
        Query(userstore): Query<QueryUserStore>,
        State(pool): State<DbPool>,
) -> Result<Json<Vec<UserStore>>, ApiResponse> {
    let conn = pool.get().await.map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;

    let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)>
        = Vec::new();

    let query = match (
        userstore.store_id.as_ref(),
        userstore.user_id.as_ref(),
        userstore.role.as_ref()) {
        (Some(store_id), Some(user_id), Some(role)) => {
            params.push(store_id);
            params.push(user_id);
            params.push(role);
            "SELECT * FROM user_store WHERE store_id = $1 AND user_id = $2 AND role = $3"
        }
        (Some(store_id), Some(user_id), None) => {
            params.push(store_id);
            params.push(user_id);
            "SELECT * FROM user_store WHERE store_id = $1 AND user_id = $2"
        }
        (Some(store_id), None, Some(role)) => {
            params.push(store_id);
            params.push(role);
            "SELECT * FROM user_store WHERE store_id = $1 AND role = $2"
        }
        (None, Some(user_id), Some(role)) => {
            params.push(user_id);
            params.push(role);
            "SELECT * FROM user_store WHERE user_id = $1 AND role = $2"
        }
        (Some(store_id), None, None) => {
            params.push(store_id);
            "SELECT * FROM user_store WHERE store_id = $1"
        }
        (None, Some(user_id), None) => {
            params.push(user_id);
            "SELECT * FROM user_store WHERE user_id = $1"
        }
        (None, None, Some(role)) => {
            params.push(role);
            "SELECT * FROM user_store WHERE role = $1"
        }
        (None, None, None) => {
            return Err(ApiResponse::err(StatusCode::BAD_REQUEST));
        }
    };

    let rows = conn.query(query, &params).await.map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;

    let userstores: Vec<UserStore> = rows.iter().map(UserStore::from).collect();

    Ok(Json(userstores))
}

pub fn make_userstore_routes() -> Router<DbPool> {
    let public_routes = Router::new()
        .route("/userstore", get(get_userstore));

    let protected_routes = Router::new()
        .route("/userstore",
            post(create_userstore)
            .put(update_userstore)
            .delete(delete_userstore))
            .layer(middleware::from_fn(jwt_middleware));

    public_routes.merge(protected_routes)
}