use crate::{
    auth::jwt_auth::Claims
};

/*
    Verifica se um usuáro pode modificar um dado perfil. Um usuário pode
    modificar seu próprio perfil e administradores podem modificar qualquer
    perfil.
*/
pub fn user_auth(
    claims: Claims,
    user_id: i64
) -> bool {
    claims.sub == user_id || claims.is_admin
}