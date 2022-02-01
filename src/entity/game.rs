use chrono::{FixedOffset, Utc};
use sea_orm::entity::prelude::*;
use sea_orm::{DatabaseConnection, DbErr, Set};
use sea_orm::{QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "game")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub uuid: Uuid,
    pub created_at: DateTimeWithTimeZone,
    pub is_against_ai: bool,
    pub player1_key: Option<Uuid>,
    pub player2_key: Option<Uuid>,
    pub winner_key: Option<Uuid>,
    pub ended_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::board::Entity")]
    Board,
}

impl Related<super::board::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Board.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub async fn create(
    creator_key: Uuid,
    conn: &DatabaseConnection,
    is_against_ai: bool,
) -> Result<Model, DbErr> {
    let game = ActiveModel {
        uuid: Set(Uuid::new_v4()),
        created_at: Set(Utc::now().with_timezone(&FixedOffset::east(0))),
        is_against_ai: Set(is_against_ai),
        player1_key: Set(Some(creator_key)),
        ..Default::default()
    };

    let game = game.insert(conn).await?;

    // create initial board for this game
    super::board::create_initial(game.uuid, conn).await?;

    Ok(game)
}

pub async fn find_by_id(game_id: Uuid, conn: &DatabaseConnection) -> Result<Option<Model>, DbErr> {
    Entity::find_by_id(game_id).one(conn).await
}

pub async fn get_last_board(
    game: &Model,
    conn: &DatabaseConnection,
) -> Result<Option<super::board::Model>, DbErr> {
    game.find_related(super::board::Entity)
        .order_by_desc(super::board::Column::CreatedAt)
        .limit(1)
        .one(conn)
        .await
}
