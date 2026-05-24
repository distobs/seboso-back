use axum::{
        Json, Router, extract::{Query, State}, http::StatusCode, routing::get
};

use crate::{models::userstore_model::{QueryUserStore, UserStore}, types::{db_types::DbPool, response_types::ApiResponse}};

async fn get_userstore(
        Query(userstore): Query<QueryUserStore>,
        State(pool): State<DbPool>,
) -> Result<Json<Vec<UserStore>>, ApiResponse> {
    let conn = pool.get().await.map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;

    if userstore.store_id.is_some() && userstore.user_id.is_some() {
        let rows = conn.query(
                "SELECT * FROM userstore WHERE user_id = $1 AND store_id = $2",
                &[&Some(userstore.user_id), &Some(userstore.store_id)])
                .await.map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;

        let relations: Vec<UserStore> =
                rows.iter().map(UserStore::from).collect();

        return Ok(Json(relations));
    } else if userstore.store_id.is_some() && userstore.user_id.is_none() {
        let rows = conn.query(
                "SELECT * FROM userstore WHERE store_id = $1",
                        &[&Some(userstore.store_id)])
                        .await
                        .map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;

        let relations: Vec<UserStore> =
                rows.iter().map(UserStore::from).collect();

        return Ok(Json(relations));
    } else if userstore.store_id.is_none() && userstore.user_id.is_some() {
        let rows = conn.query(
                "SELECT * FROM userstore WHERE user_id = $1",
                &[&Some(userstore.user_id)])
                .await.map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;
        
        let relations: Vec<UserStore> =
                rows.iter().map(UserStore::from).collect();

        return Ok(Json(relations));
    } else {
        return Err(ApiResponse::err(StatusCode::BAD_REQUEST));
    }
}

pub fn make_userstore_routes() -> Router<DbPool> {
        Router::new()
        .route("/userstore", get(get_userstore))
}