use crate::{
    auth::jwt_auth::Claims,
    models::userstore_model::UserStore,
    types::db_types::DbPool,
    utils::error_utils::DynError
};

/*
    Verifica se um usuáro pode modificar um dado sebo. Um usuário pode
    modificar sebos em que é funcionário e administradores podem modificar
    qualquer sebo.
*/
pub async fn store_auth(
    claims: &Claims,
    store_id: i64,
    pool: &DbPool,
) -> Result<bool, DynError> {
    if claims.is_admin {
        return Ok(true);
    }

    let relation = UserStore::from_user_store_id(
        pool,
        claims.sub,
        store_id
    ).await?;
    
    match relation {
        Some(relation) =>
            Ok(relation.role == "worker" || relation.role == "owner"),
        None => Ok(false)
    }
}