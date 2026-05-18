use crate::{
    auth::jwt_auth::Claims
};

/*
    Verifica se um usuáro pode manipular livros. Apenas administradores
    podem adicionar modificar livros,
*/
pub async fn book_auth(
    claims: &Claims,
) -> bool {
    claims.is_admin
}