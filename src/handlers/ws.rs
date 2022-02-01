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

                        // try playing as human
                        // break out of game loop if it fails
                        // (it only fails if game is already over)
                        if let Err(_) = play(
                            true,
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

                        // try playing as ai if in game with ai
                        // break out of game loop if it fails
                        // (it only fails if game is already over)
                        if game.is_against_ai {
                            if let Err(_) = play(
                                false,
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

async fn play(
    is_human: bool,
    game_id: Uuid,
    conn: &DatabaseConnection,
    row: usize,
    col: usize,
    player_num: u8,
    channel_tx: broadcast::Sender<String>,
    cookies: Cookies,
) -> Result<(), String> {
    // refresh game from db
    let game = entity::game::find_by_id(game_id, &conn)
        .await
        .unwrap()
        .unwrap();

    // has game already ended?
    if let Some(_) = game.ended_at {
        return Err(String::from("game already ended"));
    }

    let game_board = entity::game::get_most_recent_board(&game, conn)
        .await
        .unwrap()
        .unwrap();
    let board_state: Vec<Vec<u8>> = serde_json::from_value(game_board.state.clone()).expect(
        &format!("could not deserialize game board:\n{:?}", game_board.state),
    );

    let player_num = match is_human {
        true => player_num,
        _ => match player_num {
            1 => 2,
            2 => 1,
            _ => panic!("This shouldn't be happening!"),
        },
    };

    let (row, col) = match is_human {
        true => (row, col),
        _ => get_ai_play(&board_state, row, col),
    };

    // invalid selection?
    // TO-DO
    // * selection has already been made on this board
    // * selection goes against board rules)

    // create board for current game play
    let game_board = entity::board::create_next(game.uuid, board_state, row, col, player_num, conn)
        .await
        .map_err(|err| format!("Database error: {}", err))?;
    let board_state: Vec<Vec<u8>> = serde_json::from_value(game_board.state.clone()).expect(
        &format!("could not deserialize game board:\n{:?}", game_board.state),
    );

    // was it a winning move?
    if is_winning_move(row, col, &board_state) {
        let mut game: entity::game::ActiveModel = game.into();

        game.winner_key = Set(Some(cookies.session_id));
        game.ended_at = Set(Some(Utc::now().with_timezone(&FixedOffset::east(0))));
        game.update(conn).await.unwrap();

        let _ = channel_tx.send(format!("End {:?}", player_num));
    }

    // are there any more moves left on board?
    // TO-DO

    // notify channel of updated board
    let _ = channel_tx.send(format!("Board {:?}", board_state));

    Ok(())
}

fn get_ai_play(board: &Vec<Vec<u8>>, _row: usize, _col: usize) -> (usize, usize) {
    // _row and _col identify the last cell that was played by human opponent
    // may use this information in a future version to make ai smarter
    for i in 0..board.len() {
        if board[i][board[i].len() - 1] == 0 {
            return (i, board[i].len() - 1);
        }
    }

    (0, 0)
}
