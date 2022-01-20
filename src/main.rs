use std::env;
use std::net::SocketAddr;

use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect};
use axum::routing::{get, get_service};
use axum::{AddExtensionLayer, Router, Server};
use chrono::{FixedOffset, Utc};
use dotenv::dotenv;
use sea_orm::prelude::*;
use sea_orm::{Database, DatabaseConnection, Set};
use tera::{Context, Tera};
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use url::Url;
use uuid::Uuid;

mod cookies;
mod entity;

use cookies::Cookies;
use entity::game;

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

    let base_url = env::var("BASE_URL").expect("BASE_URL is not set in environment");
    let base_url = Url::parse(&base_url).expect("Error parsing BASE_URL");

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in environment");
    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    entity::setup::create_game_table(&conn)
        .await
        .expect("Cannot create game table");

    let app = Router::new()
        .route("/", get(index).post(create_game))
        .route("/game/:uuid/share", get(share_game))
        .route("/game/:uuid/play", get(play_game))
        .nest("/static", staticfiles_service)
        .layer(AddExtensionLayer::new(base_url))
        .layer(AddExtensionLayer::new(conn))
        .layer(AddExtensionLayer::new(templates))
        .layer(CookieManagerLayer::new());

    let address = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::debug!("listening on {}...", address);

    Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index(
    Extension(ref templates): Extension<Tera>,
    _cookies: Cookies,
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

async fn create_game(Extension(ref conn): Extension<DatabaseConnection>) -> impl IntoResponse {
    let game = game::ActiveModel {
        uuid: Set(Uuid::new_v4()),
        created_at: Set(Utc::now().with_timezone(&FixedOffset::east(0))),
    };

    let game = game.insert(conn).await.expect("cannot create game");
    let path = format!("/game/{}/share", game.uuid);

    Redirect::to(path.parse().unwrap())
}

async fn share_game(
    Path(game_id): Path<Uuid>,
    Extension(ref conn): Extension<DatabaseConnection>,
    Extension(ref templates): Extension<Tera>,
    Extension(ref base_url): Extension<Url>,
    _cookies: Cookies,
) -> Result<Html<String>, (StatusCode, String)> {
    let _game = game::Entity::find_by_id(game_id)
        .one(conn)
        .await
        .expect("game not found")
        .unwrap();

    let path = format!("game/{}/play", game_id);
    let game_url = base_url.join(&path).expect("cannot create game play url");

    let mut context = Context::new();
    context.insert("game_url", &game_url);
    context.insert("site_name", "Stacky Sides");
    let body = templates
        .render("game/share.html.tera", &context)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Template error"),
            )
        })?;

    Ok(Html(body))
}

async fn play_game(
    Path(game_id): Path<Uuid>,
    Extension(ref conn): Extension<DatabaseConnection>,
    Extension(ref templates): Extension<Tera>,
    _cookies: Cookies,
) -> Result<Html<String>, (StatusCode, String)> {
    let _game = game::Entity::find_by_id(game_id)
        .one(conn)
        .await
        .expect("game not found")
        .unwrap();

    let mut context = Context::new();
    context.insert("site_name", "Stacky Sides");
    context.insert("dim", &(0..7).collect::<Vec<usize>>());
    let body = templates
        .render("game/play.html.tera", &context)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Template error"),
            )
        })?;

    Ok(Html(body))
}
