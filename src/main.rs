use std::net::SocketAddr;

use axum::extract::{Extension, Form};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect};
use axum::routing::{get, get_service};
use axum::{AddExtensionLayer, Router, Server};
use serde::Deserialize;
use tera::{Context, Tera};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let templates = match Tera::new("templates/**/*.html.tera") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let staticfiles_service =
        get_service(ServeDir::new("./static/")).handle_error(|error: std::io::Error| async move {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", error),
            )
        });

    let app = Router::new()
        .route("/", get(index).post(create_game))
        .route("/share", get(share_game))
        .nest("/static", staticfiles_service)
        .layer(AddExtensionLayer::new(templates));

    let address = SocketAddr::from(([127, 0, 0, 1], 8000));
    Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index(
    Extension(ref templates): Extension<Tera>,
) -> Result<Html<String>, (StatusCode, String)> {
    let mut context = Context::new();
    context.insert("site_name", "Stacky Sides");
    let body = templates
        .render("game/index.html.tera", &context)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Template error"),
            )
        })?;

    Ok(Html(body))
}

async fn create_game(Form(payload): Form<GameCreationPayload>) -> impl IntoResponse {
    println!("NAME: {:?}", payload);
    Redirect::to("share".parse().unwrap())
}

async fn share_game() -> Result<Html<String>, (StatusCode, String)> {
    Ok(Html(String::from("4shared")))
}

#[derive(Deserialize, Debug)]
struct GameCreationPayload {}
