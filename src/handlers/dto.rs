// data transfer objects

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GameCreationPayload {
    pub is_against_ai: Option<bool>,
}
