use sqlx::sqlite::SqliteRow;
use sqlx::Row;
use sqlx::{Connection, SqliteConnection};

mod models;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Another option is to use SqlitePool. A pool gives a bunch of active connections and will
    //  resolve a connection from the pool when an database operation starts.
    let mut conn = SqliteConnection::connect("sqlite::memory:").await?;

    // fetch_one() requires any struct that implements the Executor trait.
    // Both Connection and Pool implement them.
    // SqliteConnection implements the Connection trait (source: https://github.com/launchbadge/sqlx/blob/028084bce3a741e995c3e6c559c6dbb27a62534d/sqlx-sqlite/src/connection/mod.rs#L198C1-L198C39)
    // Likewise, SqlitePool implements the Pool trait.
    //
    // fetch_one() function requires a mutable connection as it updates the internal state of the connection
    //  e.g. it advances the cursor which is part of the connection
    // Using a pool, on the other hand, does not require the pool to be mutable, as it gives a set of mutable connections
    //  for the client to use.
    let row: SqliteRow = sqlx::query("SELECT ?")
        .bind(150_i64)
        .fetch_one(&mut conn)
        .await?;

    // Row trait is used to access dynamic methods of SqliteRow (e.g. get()).
    //  Source: https://github.com/launchbadge/sqlx/issues/44#issuecomment-573306939
    let row = row.get::<i64, _>(0);
    assert_eq!(row, 150);

    // To map results to custom structs, use .map() function or query_as() function.
    // Please refer https://github.com/launchbadge/sqlx?tab=readme-ov-file#querying

    Ok(())
}
