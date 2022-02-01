use chrono::{FixedOffset, Utc};
use sea_orm::entity::prelude::*;
use sea_orm::{DatabaseConnection, DbErr, Set};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "board")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub game_id: Uuid,
    pub created_at: DateTimeWithTimeZone,
    pub state: Json,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::game::Entity",
        from = "Column::GameId",
        to = "super::game::Column::Uuid"
    )]
    Game,
}

impl Related<super::game::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Game.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub async fn create_next(
    game_id: Uuid,
    parent_state: Vec<Vec<u8>>,
    row: usize,
    col: usize,
    player_num: u8,
    conn: &DatabaseConnection,
) -> Result<Model, DbErr> {
    // assumes that row and column obey game rules
    // with respect to parent state
    let mut state = parent_state;
    state[row][col] = player_num;

    let board = ActiveModel {
        game_id: Set(game_id),
        state: Set(json!(state)),
        created_at: Set(Utc::now().with_timezone(&FixedOffset::east(0))),
        ..Default::default()
    };

    board.insert(conn).await
}

pub async fn create_initial(game_id: Uuid, conn: &DatabaseConnection) -> Result<Model, DbErr> {
    create_next(game_id, init_state(), 0, 0, 0, conn).await
}

pub fn init_state() -> Vec<Vec<u8>> {
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
