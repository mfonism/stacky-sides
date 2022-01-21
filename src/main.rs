use std::env;
use std::net::SocketAddr;

use axum::routing::{get, get_service};
use axum::{AddExtensionLayer, Router, Server};
use dotenv::dotenv;
use sea_orm::Database;
use tera::Tera;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use url::Url;

mod cookies;
mod entity;
mod handlers;

use entity::setup as entity_setup;
use handlers::error::handle_staticfiles_server_error;
use handlers::http::{create_game, index, play_game, share_game};
use handlers::ws::{ws_play_game, GamingChannels};

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
        get_service(ServeDir::new("./static/")).handle_error(handle_staticfiles_server_error);

    let base_url = env::var("BASE_URL").expect("BASE_URL is not set in environment");
    let base_url = Url::parse(&base_url).expect("Error parsing BASE_URL");

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in environment");
    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    entity_setup::create_game_table(&conn)
        .await
        .expect("Cannot create game table");

    let app = Router::new()
        .route("/", get(index).post(create_game))
        .route("/game/:uuid/share", get(share_game))
        .route("/game/:uuid/play", get(play_game))
        .route("/ws/game/:uuid/play", get(ws_play_game))
        .nest("/static", staticfiles_service)
        .layer(AddExtensionLayer::new(base_url))
        .layer(AddExtensionLayer::new(conn))
        .layer(AddExtensionLayer::new(GamingChannels::new_in_arc()))
        .layer(AddExtensionLayer::new(templates))
        .layer(CookieManagerLayer::new());

    let address = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::debug!("listening on {}...", address);

    Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
