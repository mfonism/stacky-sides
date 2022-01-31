use chrono::{FixedOffset, Utc};
use sea_orm::entity::prelude::*;
use sea_orm::{DatabaseConnection, DbErr, Set};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "game")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub uuid: Uuid,
    pub created_at: DateTimeWithTimeZone,
    pub player1_key: Option<Uuid>,
    pub player2_key: Option<Uuid>,
    pub board: Json,
    pub winner_key: Option<Uuid>,
    pub ended_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub async fn create(creator_key: Uuid, conn: &DatabaseConnection) -> Result<Model, DbErr> {
    let game = ActiveModel {
        uuid: Set(Uuid::new_v4()),
        created_at: Set(Utc::now().with_timezone(&FixedOffset::east(0))),
        player1_key: Set(Some(creator_key)),
        board: Set(json!(init_game_board())),
        ..Default::default()
    };

    game.insert(conn).await
}

pub async fn find_by_id(game_id: Uuid, conn: &DatabaseConnection) -> Result<Option<Model>, DbErr> {
    Entity::find_by_id(game_id).one(conn).await
}

pub fn init_game_board() -> Vec<Vec<u8>> {
    vec![
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0],
    ]
}
