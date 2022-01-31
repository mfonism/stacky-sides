use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tokio::sync::broadcast;
use uuid::Uuid;

type GameID = Uuid;

pub struct GameChannels {
    channels: Mutex<HashMap<GameID, broadcast::Sender<String>>>,
}

impl GameChannels {
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

    pub fn ensure_channel(&self, game_id: GameID) -> broadcast::Sender<String> {
        match self.query_channel(&game_id) {
            Some(channel) => channel,
            _ => {
                self.insert_channel(game_id);
                self.query_channel(&game_id).unwrap()
            }
        }
    }
}
