use crate::{auth::{jwt_auth::Claims, store_auth::store_auth}, types::db_types::DbPool, utils::error_utils::DynError};

/*
    Verifica se um usuáro pode modificar um dado catálogo. Um usuário pode
    modificar o catálogo de sebos em que é funcionário e administradores podem
    modificar qualquer catálogo.
*/
pub async fn catalog_auth(
    claims: &Claims,
    store_id: i64,
    pool: &DbPool
) -> Result<bool, DynError> {
    store_auth(claims, store_id, pool).await
}