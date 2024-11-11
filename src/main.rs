use labwhere::db::init_db;
use labwhere::models::location_type::LocationType;

// Any module that is imported into here (e.g., `use abc_module;`) has its ancestry as the binary
// crate. Therefore, any function that is declared in the module (e.g., `abc_module`) under `pub(crate)`
// visibility can be accessed by the binary crate and NOT the library crate. If the module needs to be accessed
// by both crates, it needs to be made `pub`. The binary crate depends on the library crate (which has the same 
// name listed in Cargo.toml).

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Another option is to use SqlitePool. A pool gives a bunch of active connections and will
    //  resolve a connection from the pool when an database operation starts.
    let mut conn = init_db("sqlite::memory:").await.unwrap();

    sqlx::query("INSERT INTO LOCATION_TYPES (id, name) VALUES (?, ?)")
        .bind(150_i64)
        .bind("Freezer")
        .execute(&mut conn)
        .await?;

    let result: Vec<LocationType> =
        sqlx::query_as::<_, LocationType>("SELECT * FROM LOCATION_TYPES")
            .fetch_all(&mut conn)
            .await?;

    println!("{:?}", result);

    assert_eq!(result.len(), 1);

    Ok(())
}
