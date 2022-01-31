use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Extension, Path};
use axum::response::IntoResponse;
use chrono::{FixedOffset, Utc};
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use sea_orm::prelude::*;
use sea_orm::{DatabaseConnection, Set};
use serde_json;
use serde_json::json;
use tokio::sync::broadcast;
use uuid::Uuid;

use super::message::GameMessage;
use super::utils::is_winning_move;
use crate::channels::GameChannels;
use crate::cookies::Cookies;
use crate::entity;

pub async fn ws_play_game(
    ws: WebSocketUpgrade,
    cookies: Cookies,
    Path(game_id): Path<Uuid>,
    Extension(conn): Extension<DatabaseConnection>,
    Extension(game_channels): Extension<Arc<GameChannels>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| ws_game_play_handler(socket, conn, game_id, game_channels, cookies))
}

async fn ws_game_play_handler(
    stream: WebSocket,
    conn: DatabaseConnection,
    game_id: Uuid,
    game_channels: Arc<GameChannels>,
    cookies: Cookies,
) {
    let (mut own_tx, mut own_rx) = stream.split();

    let game = entity::game::find_by_id(game_id, &conn)
        .await
        .expect("database error in finding game")
        .expect(&format!("could not find game: {}", game_id));

    // get player number
    // 1 -- player 1, black
    // 2 -- player 2, white
    // 0 -- observer
    let mut player_num = 0;
    // check whether they are player 1
    if let Some(key1) = game.player1_key {
        if key1 == cookies.session_id {
            player_num = 1;
        }
    }
    // if they're still an observer
    // check whether they are player 2
    if player_num == 0 {
        if let Some(key2) = game.player2_key {
            if key2 == cookies.session_id {
                player_num = 2;
            }
        }
    }

    // subscribe to receive messages in gaming channel
    let channel_tx = game_channels.ensure_channel(game.uuid);
    let mut channel_rx = channel_tx.subscribe();

    // Task for receiving broadcast messages from the channel
    // and possibly sending them back to own client
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = channel_rx.recv().await {
            if let Ok(msg) = GameMessage::read(msg) {
                match msg {
                    GameMessage::Board { state_str } => {
                        if own_tx.send(Message::Text(state_str)).await.is_err() {
                            break;
                        }
                    }
                    GameMessage::End { ending_str } => {
                        if own_tx.send(Message::Text(ending_str)).await.is_err() {
                            break;
                        }
                    }
                    _ => {}
                }
            }
        }
    });

    let channel_tx = channel_tx.clone();
    let player_num = player_num.clone();

    // Task for receiving messages from own client
    // and possibly broadcasting to channel
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = own_rx.next().await {
            if let Ok(msg) = GameMessage::read(text) {
                match msg {
                    GameMessage::Selection { row, col } => {
                        let row = row as usize;
                        let col = col as usize;

                        // not a player?
                        if player_num == 0 {
                            return;
                        }

                        if let Err(_) = play_as_human(
                            game_id,
                            &conn,
                            row,
                            col,
                            player_num,
                            channel_tx.clone(),
                            cookies,
                        )
                        .await
                        {
                            break;
                        }

                        if game.is_against_ai {
                            if let Err(_) =
                                play_as_ai(game_id, &conn, player_num, channel_tx.clone(), cookies)
                                    .await
                            {
                                break;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    });

    // If any one of the tasks exit, abort the other
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}

async fn play_as_human(
    game_id: Uuid,
    conn: &DatabaseConnection,
    row: usize,
    col: usize,
    player_num: u8,
    channel_tx: broadcast::Sender<String>,
    cookies: Cookies,
) -> Result<(), &str> {
    // refresh game from db
    let game = entity::game::find_by_id(game_id, &conn)
        .await
        .unwrap()
        .unwrap();

    play(
        game,
        &conn,
        row,
        col,
        player_num,
        channel_tx.clone(),
        cookies,
    )
    .await
}

async fn play_as_ai(
    game_id: Uuid,
    conn: &DatabaseConnection,
    player_num: u8,
    channel_tx: broadcast::Sender<String>,
    cookies: Cookies,
) -> Result<(), &str> {
    // refresh game from db
    let game = entity::game::find_by_id(game_id, &conn)
        .await
        .unwrap()
        .unwrap();

    let player_num = match player_num {
        1 => 2,
        2 => 1,
        _ => panic!("This shouldn't be happening!"),
    };

    let board: Vec<Vec<u8>> = serde_json::from_value(game.board.clone()).expect(&format!(
        "could not deserialize game board:\n{:?}",
        game.board
    ));
    let (row, col) = get_ai_play(board);

    play(
        game,
        &conn,
        row,
        col,
        player_num,
        channel_tx.clone(),
        cookies,
    )
    .await
}

async fn play(
    game: entity::game::Model,
    conn: &DatabaseConnection,
    row: usize,
    col: usize,
    player_num: u8,
    channel_tx: broadcast::Sender<String>,
    cookies: Cookies,
) -> Result<(), &str> {
    // game has already ended
    if let Some(_) = game.ended_at {
        return Err("game already ended");
    }

    // invalid selection?
    // TO-DO
    // * selection has already been made on this board
    // * selection goes against board rules)

    // update game board with incoming selection
    let game_board = game.board.clone();
    let mut game_board: Vec<Vec<u8>> = serde_json::from_value(game_board).expect(&format!(
        "could not deserialize game board:\n{:?}",
        game.board
    ));
    game_board[row][col] = player_num;

    let mut game: entity::game::ActiveModel = game.into();
    game.board = Set(json!(game_board));

    // was it a winning move?
    if is_winning_move(row, col, &game_board) {
        game.winner_key = Set(Some(cookies.session_id));
        game.ended_at = Set(Some(Utc::now().with_timezone(&FixedOffset::east(0))));
        let _ = channel_tx.send(format!("End {:?}", player_num));
    }

    // are there any more moves left on board?
    // TO-DO

    game.update(conn).await.unwrap();

    // notify channel of updated board
    let _ = channel_tx.send(format!("Board {:?}", game_board));

    Ok(())
}

fn get_ai_play(board: Vec<Vec<u8>>) -> (usize, usize) {
    for i in 0..board.len() {
        if board[i][board[i].len() - 1] == 0 {
            return (i, board[i].len() - 1);
        }
    }

    (0, 0)
}
