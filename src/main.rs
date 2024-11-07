use labwhere::db::init_db;
use labwhere::models::location_type::LocationType;

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
