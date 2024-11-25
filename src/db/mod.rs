use sqlx::{Connection, Error, SqliteConnection};
use std::fs;

pub mod create_db;
pub mod savable;

/// Initializes a test database and injects the schemas.
///
/// The visibility of this function **cannot** be made `pub(crate)`` as the ancestry hierarchy of this module is is follows:
///     `db -> labwhere (lib)``.
/// Therefore, making the function visibility from `pub` to `pub(crate)` will make this **only** available to
/// the lib crate (which is under the same name as `labwhere`) but not to the binary crate. Because this initialization
/// logic needs to run in our binary executable as well (upon application startup), we will keep this function visibility
/// as `pub`.
///
/// Example usage:
/// ```
/// #[tokio::test]
/// async fn test_create_location_type() {
///    let mut conn = init_db("sqlite::memory:").await.unwrap();
///    let insert_query_result = sqlx::query("INSERT INTO LOCATION_TYPES (id, name) VALUES (?, ?)")
///         .bind(150_i64)
///         .bind("Freezer")
///         .execute(&mut conn)
///         .await;
///     let location_types_result =
///     sqlx::query_as::<_, LocationType>("SELECT * FROM LOCATION_TYPES")
///         .fetch_all(&mut conn)
///         .await;
///     let location_types = location_types_result.unwrap();
///     assert_eq!(location_types.len(), 1);
/// }
pub async fn init_db(url: &str) -> Result<SqliteConnection, Error> {
    let mut connection = SqliteConnection::connect(url).await?;
    let schemas =
        fs::read_to_string("./src/db/schema.sql").expect("Something went wrong reading the file");
    sqlx::query(&schemas).execute(&mut connection).await?;
    Ok(connection)
}
