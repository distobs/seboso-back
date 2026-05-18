use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Serialize};

#[derive(Serialize, Debug)]
pub struct ApiResponse {
    pub success: bool,
    pub message: String,

    #[serde(skip)]
    pub status: Option<StatusCode>,
}

impl From<tokio_postgres::Error> for ApiResponse {
    fn from(db_err: tokio_postgres::Error) -> Self {
        eprintln!("Database error: {:?}", db_err);

        Self {
            success: false,
            message: "Database error.".to_string(),
            status: Some(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl ApiResponse {
    // Adds an automatic success message
    pub fn ok() -> Self {
        Self {
            success: true,
            message: "Sucesso.".to_string(),
            status: None,
        }
    }

    // Adds a personalized success message
    pub fn ok_msg(message: impl Into<String>) -> Self {
        Self {
            success: true,
            message: message.into(),
            status: None,
        }
    }

    // Adds an automatic message to errors based on the status code
    pub fn err(status: StatusCode) -> Self {
        Self {
            success: false,
            message: match status {
                StatusCode::FORBIDDEN => "Permissões insuficientes.".to_string(),
                StatusCode::UNAUTHORIZED => "Autenticação necessária.".to_string(),
                StatusCode::NOT_FOUND => "Recurso não encontrado.".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR => "Erro interno do servidor.".to_string(),
                _ => "Requisição inválida.".to_string(),
            },
            status: Some(status),
        }
    }

    // Adds a personalized error message
    pub fn err_msg(message: impl Into<String>, status: StatusCode) -> Self {
        Self {
            success: false,
            message: message.into(),
            status: Some(status),
        }
    }

    // Internal database error
    pub fn db_error(db_err: tokio_postgres::Error) -> Self {
        eprintln!("Database error: {:?}", db_err);

        Self {
            success: false,
            message: "Database error.".to_string(),
            status: Some(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        let status = if self.success {
            StatusCode::OK
        } else {
            self.status.unwrap_or(StatusCode::BAD_REQUEST)
        };

        (status, Json(self)).into_response()
    }
}