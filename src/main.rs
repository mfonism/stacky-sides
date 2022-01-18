use std::env;
use std::net::SocketAddr;

use axum::extract::{Extension, Form};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect};
use axum::routing::{get, get_service};
use axum::{AddExtensionLayer, Router, Server};
use chrono::{FixedOffset, Utc};
use dotenv::dotenv;
use sea_orm::prelude::*;
use sea_orm::{Database, DatabaseConnection, Set};
use serde::Deserialize;
use tera::{Context, Tera};
use tower_http::services::ServeDir;
use uuid::Uuid;

mod entities;

#[tokio::main]
async fn main() {
    dotenv().ok();

    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "debug");
    }
    tracing_subscriber::fmt::init();

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

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in environment");
    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    entities::setup::create_game_table(&conn)
        .await
        .expect("Cannot create game table");

    let app = Router::new()
        .route("/", get(index).post(create_game))
        .route("/share", get(share_game))
        .nest("/static", staticfiles_service)
        .layer(AddExtensionLayer::new(conn))
        .layer(AddExtensionLayer::new(templates));

    let address = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::debug!("listening on {}...", address);

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

async fn create_game(
    Form(payload): Form<GameCreationPayload>,
    Extension(ref conn): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    let game = entities::game::ActiveModel {
        uuid: Set(Uuid::new_v4()),
        created_at: Set(Utc::now().with_timezone(&FixedOffset::east(0))),
    };

    let game = game.insert(conn).await.expect("cannot create game");

    println!("{:?}", payload);
    println!("{:?}", game);
    Redirect::to("share".parse().unwrap())
}

async fn share_game() -> Result<Html<String>, (StatusCode, String)> {
    Ok(Html(String::from("4shared")))
}

#[derive(Deserialize, Debug)]
struct GameCreationPayload {}
