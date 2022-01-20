use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect};
use chrono::{FixedOffset, Utc};
use sea_orm::prelude::*;
use sea_orm::{DatabaseConnection, Set};
use tera::{Context, Tera};
use url::Url;
use uuid::Uuid;

use crate::cookies::Cookies;
use crate::entity::game;

pub async fn index(
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

pub async fn create_game(Extension(ref conn): Extension<DatabaseConnection>) -> impl IntoResponse {
    let game = game::ActiveModel {
        uuid: Set(Uuid::new_v4()),
        created_at: Set(Utc::now().with_timezone(&FixedOffset::east(0))),
        ..Default::default()
    };

    let game = game.insert(conn).await.expect("cannot create game");
    let path = format!("/game/{}/share", game.uuid);

    Redirect::to(path.parse().unwrap())
}

pub async fn share_game(
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

pub async fn play_game(
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
