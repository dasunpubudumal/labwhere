use sqlx::{Connection, Executor, SqliteConnection};
use sqlx::sqlite::SqliteRow;
use sqlx::Row;

mod models;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {

    let mut conn = SqliteConnection::connect("sqlite::memory:").await?;

    let row: SqliteRow = sqlx::query("SELECT ?")
        .bind(150_i64)
        .fetch_one(&mut conn).await?;

    let row = row.get::<i64, _>(0);

    assert_eq!(row, 150);

    Ok(())
}
