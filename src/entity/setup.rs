use sea_orm::error::DbErr;
use sea_orm::sea_query::value::Value;
use sea_orm::sea_query::{ColumnDef, TableCreateStatement};
use sea_orm::{sea_query, ConnectionTrait, DbConn, ExecResult};
use sea_query::foreign_key::{ForeignKey, ForeignKeyAction};

use super::{board, game};

async fn create_table(conn: &DbConn, stmt: &TableCreateStatement) -> Result<ExecResult, DbErr> {
    let builder = conn.get_database_backend();
    conn.execute(builder.build(stmt)).await
}

pub async fn create_game_table(conn: &DbConn) -> Result<ExecResult, DbErr> {
    let stmt = sea_query::Table::create()
        .table(game::Entity)
        .if_not_exists()
        .col(
            ColumnDef::new(game::Column::Uuid)
                .uuid()
                .not_null()
                .primary_key(),
        )
        .col(
            ColumnDef::new(game::Column::CreatedAt)
                .timestamp_with_time_zone()
                .not_null(),
        )
        .col(
            ColumnDef::new(game::Column::IsAgainstAi)
                .boolean()
                .default(Value::Bool(Some(false))),
        )
        .col(ColumnDef::new(game::Column::Player1Key).uuid())
        .col(ColumnDef::new(game::Column::Player2Key).uuid())
        .col(ColumnDef::new(game::Column::WinnerKey).uuid())
        .col(ColumnDef::new(game::Column::EndedAt).timestamp_with_time_zone())
        .to_owned();

    create_table(conn, &stmt).await
}

pub async fn create_board_table(conn: &DbConn) -> Result<ExecResult, DbErr> {
    let stmt = sea_query::Table::create()
        .table(board::Entity)
        .if_not_exists()
        .col(
            ColumnDef::new(board::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(board::Column::GameId).uuid().not_null())
        .foreign_key(
            ForeignKey::create()
                .name("FK_board_game")
                .from(board::Entity, board::Column::GameId)
                .to(game::Entity, game::Column::Uuid)
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .col(
            ColumnDef::new(board::Column::CreatedAt)
                .timestamp_with_time_zone()
                .not_null(),
        )
        .col(ColumnDef::new(board::Column::State).json().not_null())
        .to_owned();

    create_table(conn, &stmt).await
}
