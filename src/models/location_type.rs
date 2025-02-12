use sqlx::SqliteConnection;
use PartialEq;

/// LocationType struct
/// A LocationType is a type of location, e.g. Building, Room, etc.
#[derive(Debug, PartialEq, sqlx::FromRow)]
pub struct LocationType {
    /// The unique identifier for the LocationType
    pub id: u32,
    /// The unique name of the LocationType
    name: String,
}

/// Implementation of the LocationType struct
impl LocationType {
    /// Create a new LocationType
    /// # Examples
    ///
    /// ```
    /// # #[cfg(doctest)] {
    /// use location_type::LocationType;
    /// let locationType = LocationType::new(1, "Building".to_string());
    /// # }
    /// ```
    fn new(id: u32, name: String) -> LocationType {
        LocationType { id, name }
    }

    /// Create a new LocationType
    /// # Examples
    /// ```
    /// # #[cfg(doctest)] {
    /// use location_type::LocationType;
    /// let locationType = LocationType::create("Building".to_string()).await.unwrap();
    /// # }
    /// ```
    pub(crate) async fn create(
        name: String,
        connection: &mut SqliteConnection,
    ) -> Result<LocationType, sqlx::Error> {
        let insert_query_result = sqlx::query("INSERT INTO location_types (name) VALUES (?)")
            .bind(name.clone())
            .execute(&mut *connection)
            .await?;
        let id = insert_query_result.last_insert_rowid();
        Ok(LocationType::new(id as u32, name))
    }
}

impl Default for LocationType {
    fn default() -> LocationType {
        LocationType {
            id: 1,
            name: "Building".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::db::init_db;
    use crate::models::location_type::LocationType;

    #[test]
    fn test_location_type_new() {
        let location_type = LocationType::new(1, "Building".to_string());
        assert_eq!(location_type.id, 1);
        assert_eq!(location_type.name, "Building");
    }

    #[tokio::test]
    async fn test_create_location_type() {
        let mut conn = init_db("sqlite::memory:").await.unwrap();
        let location_type = LocationType::create("Freezer".to_string(), &mut conn)
            .await
            .unwrap();
        assert_eq!(location_type.id, 1);
        assert_eq!(location_type.name, "Freezer");
    }
}
