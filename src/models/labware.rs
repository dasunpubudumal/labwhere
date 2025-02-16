use super::location::UNKNOWN_LOCATION;
use crate::errors::database_error::ConnectivityError;
use crate::errors::not_found_error::NotFoundError;
use crate::errors::LabwhereError;
use crate::models::location::Location;
use sqlx::SqliteConnection;

/// Labware is stored in a location.
/// LabWhere needs to know nothing about it apart from its barcode and where it is.
/// If a labware has no location it's location will be set to unknown automatically
#[derive(Debug, PartialEq, sqlx::FromRow)]
struct Labware {
    /// The unique identifier for the Labware
    id: u32,
    /// The unique barcode of the Labware
    barcode: String,
    /// The location ID of the Labware
    location_id: u32,
}

/// Implementation of the Labware struct
impl Labware {
    /// Create a new Labware
    /// # Examples
    /// ```
    /// # #[cfg(doctest)] {
    /// use labware::Labware;
    /// let location:Location = Default::default();
    /// let labware = Labware::new(1, "trac-1".to_string(), location);
    /// # }
    /// ```
    ///
    fn new(id: u32, barcode: String, location: Option<&Location>) -> Labware {
        Labware {
            id,
            barcode,
            location_id: location.unwrap_or(&UNKNOWN_LOCATION).id,
        }
    }

    /// Create a new Labware
    /// # Examples
    /// ```
    /// # #[cfg(doctest)] {
    /// use labware::Labware;
    /// let mut connection = init_db("sqlite::memory:").await.unwrap();
    /// let labware = Labware::create("trac-1".to_string(), 1, &mut connection);
    /// # }
    /// ```
    pub(crate) async fn create(
        barcode: String,
        location_id: u32,
        connection: &mut SqliteConnection,
    ) -> Result<Labware, LabwhereError> {
        let insert_labware_result =
            sqlx::query("INSERT INTO labwares (barcode, location_id) VALUES (?, ?)")
                .bind(barcode.clone())
                .bind(location_id)
                .execute(&mut *connection)
                .await;
        match insert_labware_result {
            Ok(result) => {
                let id = result.last_insert_rowid();
                let location_result =
                    sqlx::query_as::<_, Location>("SELECT * FROM locations WHERE id = ?")
                        .bind(location_id)
                        .fetch_one(&mut *connection)
                        .await;
                match location_result {
                    Ok(location) => Ok(Labware::new(id as u32, barcode, Some(&location))),
                    Err(_) => Err(LabwhereError::NotFound(NotFoundError {
                        message: "Location not found!".to_string(),
                    })),
                }
            }
            Err(_) => Err(LabwhereError::ConnectivityError(ConnectivityError {
                message: "Error saving the labware!".to_string(),
            })),
        }
    }

    /// Updates the location of the Labware.
    /// Throws LabwhereError if
    ///     1. Location is not found.
    ///     2. Labware is not found.
    /// # Examples
    /// ```
    /// # #[cfg(doctest)] {
    ///     use labware::Labware;
    ///     let mut connection = init_db("sqlite::memory:").await.unwrap();
    ///     let mut labware = Labware::create("trac-1".to_string(), 1, &mut connection);
    ///     let location_type = LocationType::create("Freezer".to_string(), &mut conn).await.unwrap();
    ///     let location1 = Location::create("location1".to_string(), location_type.id, &mut conn).await.unwrap();
    ///     let location2 = Location::create("location1".to_string(), location_type.id, &mut conn).await.unwrap();
    ///     // Update the labware now
    ///     labware.location_id = location2.id;
    ///     let updated_labware = Labware::update(&labware, &mut connection);
    /// # }
    pub(crate) async fn update(
        labware: &Labware,
        connection: &mut SqliteConnection,
    ) -> Result<Labware, LabwhereError> {
        // Fetching the location first.
        match sqlx::query_as::<_, Location>("SELECT * FROM locations WHERE id = ?")
            .bind(labware.location_id)
            .fetch_one(&mut *connection)
            .await
        {
            // If we found the location
            Ok(location) => {
                match sqlx::query("UPDATE labwares SET location_id = ? WHERE id = ?")
                    .bind(labware.location_id)
                    .bind(labware.id)
                    .execute(&mut *connection)
                    .await
                {
                    Ok(result) => {
                        let updated_rows = result.rows_affected();
                        // If we do not find the labware.
                        if updated_rows == 0 {
                            return Err(LabwhereError::NotFound(NotFoundError {
                                message: "Labware not found!".to_string(),
                            }));
                        }
                        // If we found the labware
                        Ok(Labware::new(
                            result.last_insert_rowid() as u32,
                            labware.barcode.clone(),
                            Some(&location),
                        ))
                    }
                    // Error from the database.
                    Err(_) => Err(LabwhereError::ConnectivityError(ConnectivityError {
                        message: "Error from the database!".to_string(),
                    })),
                }
            }
            // If we do not found the location
            Err(_) => Err(LabwhereError::NotFound(NotFoundError {
                message: "Location not found!".to_string(),
            })),
        }
    }

    /// Find labware by barcode
    /// # Examples
    /// ```
    /// # #[cfg(doctest)] {
    /// use labware::Labware;
    /// let mut connection = init_db("sqlite::memory:").await.unwrap();
    /// let labware = Labware::find_by_barcode("lw-location-1", &mut connection);
    /// # }
    pub(crate) async fn find_by_barcode(
        barcode: String,
        connection: &mut SqliteConnection,
    ) -> Result<Labware, LabwhereError> {
        match sqlx::query_as::<_, Labware>("SELECT * FROM labwares WHERE barcode = ?")
            .bind(barcode)
            .fetch_one(&mut *connection)
            .await
        {
            Ok(labware) => Ok(labware),
            Err(_) => Err(LabwhereError::NotFound(NotFoundError {
                message: "Labware not found".to_string(),
            })),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::db::init_db;
    use crate::models::labware::*;
    use crate::models::location_type::LocationType;

    #[test]
    fn test_labware_new() {
        let location: Location = Default::default();
        let labware = Labware::new(1, "lw-1".to_string(), Some(&location));
        assert_eq!(labware.id, 1);
        assert_eq!(labware.barcode, "lw-1");
        assert_eq!(labware.location_id, location.id);
    }

    // We should have an equal function for the Labware struct that relies on the attributes of the
    // struct. This will allow us to compare two Labware structs and check if they are equal.
    #[test]
    fn test_labware_no_location() {
        let labware = Labware::new(1, "lw-1".to_string(), None);
        assert_eq!(labware.id, 1);
        assert_eq!(labware.barcode, "lw-1");
        assert_eq!(labware.location_id, UNKNOWN_LOCATION.as_ref().id);
    }

    #[tokio::test]
    async fn test_create_labware() {
        let mut conn = init_db("sqlite::memory:").await.unwrap();
        let location_type = LocationType::create("Freezer".to_string(), &mut conn)
            .await
            .unwrap();
        let location = Location::create("location1".to_string(), location_type.id, &mut conn)
            .await
            .unwrap();
        let labware = Labware::create("lw-1".to_string(), location.id, &mut conn)
            .await
            .unwrap();

        assert_eq!(labware.barcode, "lw-1");
        assert_eq!(labware.location_id, location.id);
    }

    #[tokio::test]
    async fn update_labware_location_that_doesnt_exist() {
        let mut conn = init_db("sqlite::memory:").await.unwrap();
        let location_type = LocationType::create("Freezer".to_string(), &mut conn)
            .await
            .unwrap();
        let location = Location::create("lw-location-1".to_string(), location_type.id, &mut conn)
            .await
            .unwrap();
        let mut labware = Labware::create("lw-1".to_string(), location.id, &mut conn)
            .await
            .unwrap();

        labware.location_id = 3;

        Labware::update(&labware, &mut conn)
            .await
            .expect_err("Location not found!");
    }

    #[tokio::test]
    async fn update_labware_that_doesnt_exist() {
        let mut conn = init_db("sqlite::memory:").await.unwrap();
        let location_type = LocationType::create("Freezer".to_string(), &mut conn)
            .await
            .unwrap();
        let location1 = Location::create("location1".to_string(), location_type.id, &mut conn)
            .await
            .unwrap();
        let location2 = Location::create("location1".to_string(), location_type.id, &mut conn)
            .await
            .unwrap();

        // This labware is not persisted in the database.
        let mut labware = Labware::new(100, "lw-20".to_string(), Some(&location1));
        labware.location_id = location2.id;
        Labware::update(&labware, &mut conn)
            .await
            .expect_err("Labware not found!");
    }

    #[tokio::test]
    async fn update_labware() {
        let mut conn = init_db("sqlite::memory:").await.unwrap();
        let location_type = LocationType::create("Freezer".to_string(), &mut conn)
            .await
            .unwrap();
        let location1 = Location::create("location1".to_string(), location_type.id, &mut conn)
            .await
            .unwrap();
        let location2 = Location::create("location2".to_string(), location_type.id, &mut conn)
            .await
            .unwrap();

        // Create the labware first.
        let mut labware = Labware::create("lw-1".to_string(), location1.id, &mut conn)
            .await
            .unwrap();

        // Update the location of the labware
        labware.location_id = location2.id;
        let updated_labware = Labware::update(&labware, &mut conn).await.unwrap();

        assert_eq!(updated_labware.barcode, "lw-1");
        assert_eq!(updated_labware.id, labware.id);
        assert_eq!(updated_labware.location_id, location2.id);
    }

    #[tokio::test]
    async fn test_find_by_barcode() {
        let mut conn = init_db("sqlite::memory:").await.unwrap();
        let location_type = LocationType::create("Freezer".to_string(), &mut conn)
            .await
            .unwrap();
        let location = Location::create("location1".to_string(), location_type.id, &mut conn)
            .await
            .unwrap();
        let labware = Labware::create("lw-1".to_string(), location.id, &mut conn)
            .await
            .unwrap();

        let fetched_labware = Labware::find_by_barcode("lw-1".to_string(), &mut conn)
            .await
            .unwrap();

        assert_eq!(labware.barcode, fetched_labware.barcode)
    }

    #[tokio::test]
    async fn test_find_by_barcode_for_not_found() {
        let mut conn = init_db("sqlite::memory:").await.unwrap();
        Labware::find_by_barcode("lw-1".to_string(), &mut conn)
            .await
            .expect_err("Labware not found");
    }
}
