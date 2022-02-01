use std::sync::Arc;

use axum::extract::{Extension, Form, Path};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect};
use sea_orm::prelude::*;
use sea_orm::{DatabaseConnection, DbErr, Set};
use tera::{Context, Tera};
use url::Url;
use uuid::Uuid;

use super::dto;
use super::error::{handle_db_error, handle_not_found_error, handle_template_error};
use crate::channels::GameChannels;
use crate::cookies::Cookies;
use crate::entity;

const SITE_NAME: &str = "Stacky Sides";

pub async fn index(
    Extension(ref templates): Extension<Tera>,
    _cookies: Cookies,
) -> Result<Html<String>, (StatusCode, String)> {
    let mut context = Context::new();
    context.insert("site_name", SITE_NAME);

    let body = templates
        .render("game/index.html.tera", &context)
        .map_err(handle_template_error)?;

    Ok(Html(body))
}

pub async fn create_game(
    Extension(ref conn): Extension<DatabaseConnection>,
    Extension(game_channels): Extension<Arc<GameChannels>>,
    Form(payload): Form<dto::GameCreationPayload>,
    cookies: Cookies,
) -> impl IntoResponse {
    let game = entity::game::create(
        cookies.session_id,
        conn,
        payload.is_against_ai.unwrap_or(false),
    )
    .await;

    if let Err(_) = &game {
        return Redirect::temporary("/".parse().unwrap());
    }

    let game = game.unwrap();
    let path = format!("/game/{}/share", game.uuid);
    game_channels.insert_channel(game.uuid);

    Redirect::to(path.parse().unwrap())
}

pub async fn share_game(
    Path(game_id): Path<Uuid>,
    Extension(ref base_url): Extension<Url>,
    Extension(ref conn): Extension<DatabaseConnection>,
    Extension(ref templates): Extension<Tera>,
    _cookies: Cookies,
) -> Result<Html<String>, (StatusCode, String)> {
    let game = entity::game::find_by_id(game_id, conn)
        .await
        .map_err(handle_db_error)?
        .ok_or(format!("Game not found: {}", game_id))
        .map_err(handle_not_found_error)?;

    let path = format!("game/{}/play", game.uuid);
    let game_url = base_url.join(&path).unwrap();

    let mut context = Context::new();
    context.insert("game_url", &game_url);
    context.insert("is_against_ai", &game.is_against_ai);
    context.insert("site_name", SITE_NAME);
    let body = templates
        .render("game/share.html.tera", &context)
        .map_err(handle_template_error)?;

    Ok(Html(body))
}

pub async fn play_game(
    Path(game_id): Path<Uuid>,
    Extension(ref conn): Extension<DatabaseConnection>,
    Extension(ref base_url): Extension<Url>,
    Extension(ref templates): Extension<Tera>,
    cookies: Cookies,
) -> Result<Html<String>, (StatusCode, String)> {
    let game = entity::game::find_by_id(game_id, conn)
        .await
        .map_err(handle_db_error)?
        .ok_or(format!("Game not found: {}", game_id))
        .map_err(handle_not_found_error)?;

    let is_against_ai = game.is_against_ai;
    let game_board = game.board.clone();
    let is_game_over = match game.ended_at {
        None => false,
        _ => true,
    };

    // assign player number
    // 1 -- player 1, black
    // 2 -- player 2, white
    // 0 -- observer
    let player_num = get_assigned_player_number(game, cookies.session_id, conn)
        .await
        .map_err(handle_db_error)?;

    let path = format!("/ws/game/{}/play", game_id);
    let game_ws_url = get_ws_url_for_path(path, base_url.clone());

    let mut context = Context::new();
    context.insert("site_name", SITE_NAME);
    context.insert("is_against_ai", &is_against_ai);
    context.insert("player_num", &player_num);
    context.insert("game_board", &game_board);
    context.insert("is_game_over", &is_game_over);
    context.insert("game_board_width", &7);
    context.insert("game_board_height", &7);
    context.insert("game_ws_url", &game_ws_url);
    let body = templates
        .render("game/play.html.tera", &context)
        .map_err(handle_template_error)?;

    Ok(Html(body))
}

async fn get_assigned_player_number(
    game: entity::game::Model,
    session_id: Uuid,
    conn: &DatabaseConnection,
) -> Result<usize, DbErr> {
    // find next unassigned `player key` in game
    // set it to session_id
    // and return whether this makes owner of session_id player 1 or 2
    // (or 0, which is what all non-playing observers are)
    let res = match (game.player1_key, game.player2_key) {
        (None, None) => assign_player(game, conn, session_id, 1).await?,
        (None, Some(key2)) => match key2 == session_id {
            true => 2,
            _ => match game.is_against_ai {
                true => 0,
                _ => assign_player(game, conn, session_id, 1).await?,
            },
        },
        (Some(key1), None) => match key1 == session_id {
            true => 1,
            _ => match game.is_against_ai {
                true => 0,
                _ => assign_player(game, conn, session_id, 2).await?,
            },
        },
        (Some(key1), Some(key2)) => match key1 == session_id {
            true => 1,
            _ => match key2 == session_id {
                true => 2,
                _ => 0,
            },
        },
    };

    Ok(res)
}

async fn assign_player(
    game: entity::game::Model,
    conn: &DatabaseConnection,
    session_id: Uuid,
    player_num: usize,
) -> Result<usize, DbErr> {
    let mut game: entity::game::ActiveModel = game.into();
    match player_num {
        1 => game.player1_key = Set(Some(session_id)),
        2 => game.player2_key = Set(Some(session_id)),
        _ => panic!("cannot assign player with key greater than 2 or less than 1"),
    }
    game.update(conn).await?;
    Ok(player_num)
}

fn get_ws_url_for_path(path: String, mut base_url: Url) -> String {
    base_url
        .set_scheme("ws")
        .expect("cannot change BASE_URL's scheme");
    let game_ws_url = base_url
        .join(&path)
        .expect("cannot create game play ws url");
    serde_json::to_string(&game_ws_url).expect("cannot serialize game play ws url")
}
