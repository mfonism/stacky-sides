use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Extension, Path};
use axum::response::IntoResponse;
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use sea_orm::prelude::*;
use sea_orm::DatabaseConnection;
use tokio::sync::broadcast;
use uuid::Uuid;

use super::utils::GameMessage;
use crate::cookies::Cookies;
use crate::entity::game::{Entity as GameEntity, Model as GameModel};

type GameID = Uuid;

pub struct GamingChannels {
    channels: Mutex<HashMap<GameID, broadcast::Sender<String>>>,
}

impl GamingChannels {
    fn new() -> Self {
        Self {
            channels: Mutex::new(HashMap::new()),
        }
    }

    pub fn new_in_arc() -> Arc<Self> {
        Arc::new(Self::new())
    }

    pub fn insert_channel(&self, game_id: GameID) -> Option<broadcast::Sender<String>> {
        let (channel_tx, _channel_rx) = broadcast::channel(100);
        self.channels.lock().unwrap().insert(game_id, channel_tx)
    }

    pub fn query_channel(&self, game_id: &GameID) -> Option<broadcast::Sender<String>> {
        if let Some(channel_tx) = self.channels.lock().unwrap().get(game_id) {
            Some(channel_tx.clone())
        } else {
            None
        }
    }
}

pub async fn ws_play_game(
    ws: WebSocketUpgrade,
    cookies: Cookies,
    Path(game_id): Path<Uuid>,
    Extension(conn): Extension<DatabaseConnection>,
    Extension(gaming_channels): Extension<Arc<GamingChannels>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| {
        ws_game_play_handler(game_id, gaming_channels, socket, conn, cookies)
    })
}

async fn ws_game_play_handler(
    game_id: Uuid,
    gaming_channels: Arc<GamingChannels>,
    stream: WebSocket,
    db_conn: DatabaseConnection,
    cookies: Cookies,
) {
    let (mut own_tx, mut own_rx) = stream.split();

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

    // subscribe to receive messages in gaming channel
    let channel_tx = gaming_channels
        .query_channel(&game_id)
        .expect("channel not found for game");
    let mut channel_rx = channel_tx.subscribe();

    // Task for receiving broadcast messages from the channel
    // and possibly sending them back to own client
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = channel_rx.recv().await {
            if own_tx
                .send(Message::Text(String::from("Hey hey")))
                .await
                .is_err()
            {
                break;
            }
        }
    });

    let channel_tx = channel_tx.clone();

    // Task for receiving messages from own client
    // and possibly broadcasting to channel
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = own_rx.next().await {
            if let Ok(msg) = GameMessage::read(text) {
                match msg {
                    GameMessage::Selection { row, col } => {
                        println!("Made a selection of {:?}, {:?}", row, col);
                        // persist this selection against game state
                        // broadcast to everyone to refresh board
                        // actually, send new board to everyone
                    }
                    GameMessage::End { winner } => {
                        println!("The winner is player {:?}", winner);
                    }
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