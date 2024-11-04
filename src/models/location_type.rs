use PartialEq;

/// LocationType struct  
/// A LocationType is a type of location, e.g. Building, Room, etc.
#[derive(Debug, PartialEq, sqlx::FromRow)]
pub struct LocationType {
    /// The unique identifier for the LocationType
    #[sqlx(rename = "ID")]
    pub id: u32,
    /// The unique name of the LocationType
    #[sqlx(rename = "NAME")]
    name: String,
}

/// Implementation of the LocationType struct
impl LocationType {
    /// Create a new LocationType
    /// # Examples
    ///
    /// ```
    /// use location_type::LocationType;
    /// let locationType = LocationType::new(1, "Building".to_string());
    /// ```
    fn new(id: u32, name: String) -> LocationType {
        LocationType { id, name }
    }
}

impl Default for LocationType {
    fn default() -> LocationType {
        LocationType {
            id: 1,
            name: "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_test_db;

    #[test]
    fn test_location_type_new() {
        let location_type = LocationType::new(1, "Building".to_string());
        assert_eq!(location_type.id, 1);
        assert_eq!(location_type.name, "Building");
    }

    #[tokio::test]
    async fn test_create_location_type() {
        let mut conn = init_test_db().await.unwrap();
        let insert_query_result =
            sqlx::query("INSERT INTO LOCATION_TYPES (id, name) VALUES (?, ?)")
                .bind(150_i64)
                .bind("Freezer")
                .execute(&mut conn)
                .await;
        let location_types_result =
            sqlx::query_as::<_, LocationType>("SELECT * FROM LOCATION_TYPES")
                .fetch_all(&mut conn)
                .await;
        let location_types = location_types_result.unwrap();
        assert_eq!(location_types.len(), 1);
    }
}
