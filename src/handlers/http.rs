use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect};
use chrono::{FixedOffset, Utc};
use sea_orm::prelude::*;
use sea_orm::{DatabaseConnection, DbErr, Set};
use tera::{Context, Tera};
use url::Url;
use uuid::Uuid;

use super::error::{handle_db_error, handle_template_error};
use crate::cookies::Cookies;
use crate::entity::game::{
    ActiveModel as GameActiveModel, Entity as GameEntity, Model as GameModel,
};

pub async fn index(
    Extension(ref templates): Extension<Tera>,
    _cookies: Cookies,
) -> Result<Html<String>, (StatusCode, String)> {
    let mut context = Context::new();
    context.insert("site_name", "Stacky Sides");
    let body = templates
        .render("game/index.html.tera", &context)
        .map_err(handle_template_error)?;

    Ok(Html(body))
}

pub async fn create_game(
    Extension(ref conn): Extension<DatabaseConnection>,
    cookies: Cookies,
) -> impl IntoResponse {
    let game: GameActiveModel = GameActiveModel {
        uuid: Set(Uuid::new_v4()),
        created_at: Set(Utc::now().with_timezone(&FixedOffset::east(0))),
        player1_key: Set(Some(cookies.session_id)), // creator is player1
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
    let _game: GameModel = GameEntity::find_by_id(game_id)
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
        .map_err(handle_template_error)?;

    Ok(Html(body))
}

pub async fn play_game(
    Path(game_id): Path<Uuid>,
    Extension(ref conn): Extension<DatabaseConnection>,
    Extension(ref templates): Extension<Tera>,
    Extension(ref base_url): Extension<Url>,
    cookies: Cookies,
) -> Result<Html<String>, (StatusCode, String)> {
    let game: GameModel = GameEntity::find_by_id(game_id)
        .one(conn)
        .await
        .expect("game not found")
        .unwrap();

    // assign player number
    // 0 -- observer
    // 1 -- black
    // 2 -- white
    let mut player_num = 0;
    let session_id = cookies.session_id;
    match (game.player1_key, game.player2_key) {
        (None, None) => {
            player_num = 1;
            assign_player(game, conn, session_id, player_num)
                .await
                .map_err(handle_db_error)?;
        }
        (None, Some(key2)) => {
            if key2 == session_id {
                player_num = 2;
            } else {
                player_num = 1;
                assign_player(game, conn, session_id, player_num)
                    .await
                    .map_err(handle_db_error)?;
            }
        }
        (Some(key1), None) => {
            if key1 == session_id {
                player_num = 1;
            } else {
                player_num = 2;
                assign_player(game, conn, session_id, player_num)
                    .await
                    .map_err(handle_db_error)?;
            }
        }
        (Some(key1), Some(key2)) => {
            if key1 == session_id {
                player_num = 1;
            } else if key2 == session_id {
                player_num = 2;
            }
        }
    };

    let path = format!("game/{}/play", game_id);
    let game_ws_url = get_ws_url_for_path(path, base_url.clone());

    let mut context = Context::new();
    context.insert("site_name", "Stacky Sides");
    context.insert("player_num", &player_num);
    context.insert("dim", &(0..7).collect::<Vec<usize>>());
    context.insert("game_ws_url", &game_ws_url);
    let body = templates
        .render("game/play.html.tera", &context)
        .map_err(handle_template_error)?;

    Ok(Html(body))
}

async fn assign_player(
    game: GameModel,
    conn: &DatabaseConnection,
    session_id: Uuid,
    player_num: usize,
) -> Result<GameModel, DbErr> {
    let mut game: GameActiveModel = game.into();
    match player_num {
        1 => {
            game.player1_key = Set(Some(session_id));
        }
        2 => {
            game.player2_key = Set(Some(session_id));
        }
        _ => {
            panic!("cannot assign player with key greater than 2 or less than 1");
        }
    }
    let game: GameModel = game.update(conn).await?;
    Ok(game)
}

fn get_ws_url_for_path(path: String, mut base_url: Url) -> String {
    base_url
        .set_scheme("ws")
        .expect("cannot change BASE_URL's scheme");
    base_url
        .set_port(Some(3000))
        .expect("cannot change BASE_URL's port");
    let game_ws_url = base_url
        .join(&path)
        .expect("cannot create game play ws url");
    serde_json::to_string(&game_ws_url).expect("cannot serialize game play ws url")
}
