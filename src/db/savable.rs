use sqlx::SqliteConnection;

/// Trait for saving objects to the database
///
/// This trait can only be applied to structs whose size can be computed at compile time.
///
/// The visibility of this trait is confined to the library crate. Ideally, the main crate should not use
/// the savable trait as it is the library crate that should encapsulate model logic.
pub(crate) trait Savable: Sized {
    /// Saves the object to the database.
    ///
    /// Helps to add a "object-oriented" style save function for the struct.
    ///
    /// # Arguments
    ///
    /// * `self` - The object to be saved.
    /// * `conn` - A `SqliteConnection` to the database.
    ///
    /// # Returns
    ///
    /// Returns a future that resolves to a `Result` containing the saved object or a `sqlx::Error`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(doctest)] {
    /// use sqlx::sqlite::SqliteConnection;
    /// use labwhere::db::savable::Savable;
    /// use labwhere::db::init_db;
    /// use labwhere::models::location_type::LocationType;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), sqlx::Error> {
    ///     let mut conn = init_db("sqlite::memory:").await.unwrap();
    ///
    ///     // Create the LOCATION_TYPES table
    ///     sqlx::query("CREATE TABLE LOCATION_TYPES (id INTEGER PRIMARY KEY, name TEXT NOT NULL)")
    ///         .execute(&mut conn)
    ///         .await?;
    ///
    ///     let location_type = LocationType::new(0, "Warehouse".to_string());
    ///     let saved_location_type = location_type.save(conn).await?;
    ///
    ///     assert_eq!(saved_location_type.id, 1);
    ///     assert_eq!(saved_location_type.name, "Warehouse");
    ///
    ///     Ok(())
    /// }
    /// # }
    fn save(
        &self,
        conn: SqliteConnection,
    ) -> impl std::future::Future<Output = Result<Self, sqlx::Error>> + Send;
}
