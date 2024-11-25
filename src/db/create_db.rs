use sqlx::migrate::MigrateDatabase;

/// Creates an SQLite database.
///
/// # Arguments
///
/// * `path` - The folder where the database will be created.
/// * `environment` - The environment to create the database in e.g. test, dev, prod.
///
/// # Returns
/// Returns a `Result` containing `()` or a `sqlx::Error`.
///
/// Will create a database in the folder with the environment name
///
/// # Examples
/// ```
/// # #[cfg(doctest)] {
/// create_db("src/db", "test").await;
/// }
/// ```
pub async fn create_db(path: Option<&str>, environment: &str) -> Result<(), sqlx::Error> {
    let url = match path {
        Some(path) => {
            format!("sqlite://{}/{}.db", path, environment)
        }
        None => {
            format!("sqlite://{}.db", environment)
        }
    };
    sqlx::Sqlite::create_database(&url).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::db::create_db::create_db;
    use crate::db::init_db;
    use sqlx::migrate::MigrateDatabase;

    #[tokio::test]
    async fn test_create_db() {
        let result = create_db(None, "test").await;
        init_db("sqlite://test.db").await.unwrap();
        assert_eq!(result.is_ok(), true);
        sqlx::Sqlite::drop_database("sqlite://test.db")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_create_db_with_path() {
        let result = create_db(Some("src/db"), "test").await;
        init_db("sqlite://src/db/test.db").await.unwrap();
        assert_eq!(result.is_ok(), true);
        sqlx::Sqlite::drop_database("sqlite://src/db/test.db")
            .await
            .unwrap();
    }
}
