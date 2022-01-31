use axum::http::StatusCode;
use sea_orm::DbErr;
use tera::Error as TemplateError;

pub fn handle_db_error(error: DbErr) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Database error: {}", error),
    )
}

pub fn handle_template_error(error: TemplateError) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Template error: {}", error),
    )
}

pub fn handle_not_found_error(error: String) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, error)
}

pub async fn handle_staticfiles_server_error(error: std::io::Error) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Static files server error: {}", error),
    )
}
