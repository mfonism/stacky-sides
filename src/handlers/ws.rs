use axum::extract::ws::{WebSocket, WebSocketUpgrade};
use axum::extract::{Extension, Path};
use axum::response::IntoResponse;
use futures::stream::StreamExt;
use sea_orm::prelude::*;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::cookies::Cookies;
use crate::entity::game::{Entity as GameEntity, Model as GameModel};

pub async fn ws_play_game(
    ws: WebSocketUpgrade,
    cookies: Cookies,
    Path(game_id): Path<Uuid>,
    Extension(conn): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| ws_game_play_handler(game_id, socket, conn, cookies))
}

async fn ws_game_play_handler(
    game_id: Uuid,
    stream: WebSocket,
    db_conn: DatabaseConnection,
    cookies: Cookies,
) {
    let (mut own_sender, mut own_receiver) = stream.split();

    let game: GameModel = GameEntity::find_by_id(game_id)
        .one(&db_conn)
        .await
        .expect("game not found")
        .unwrap();

    let mut player_num = 0;
    if let Some(key1) = game.player1_key {
        if key1 == cookies.session_id {
            player_num = 1;
        }
    } else if let Some(key2) = game.player2_key {
        if key2 == cookies.session_id {
            player_num = 2;
        }
    }

    println!("User connected: {:?}", cookies.session_id);
    println!("Playing game: {:?}", game_id);
    println!("Player {:?}", player_num);
}
