use sea_orm::error::DbErr;
use sea_orm::sea_query::value::Value;
use sea_orm::sea_query::{ColumnDef, TableCreateStatement};
use sea_orm::{sea_query, ConnectionTrait, DbConn, ExecResult};

use super::game;

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
        .col(ColumnDef::new(game::Column::Board).json())
        .col(ColumnDef::new(game::Column::WinnerKey).uuid())
        .col(ColumnDef::new(game::Column::EndedAt).timestamp_with_time_zone())
        .to_owned();

    create_table(conn, &stmt).await
}
