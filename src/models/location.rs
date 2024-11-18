use crate::errors::NotFoundError;
use crate::models::location_type::LocationType;
use once_cell::sync::Lazy;
use regex::Regex;
use sqlx::SqliteConnection;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use PartialEq;

/// The `UNKNOWN_LOCATION` constant is initialized only when it is first accessed.
///  This can save resources if the constant is not used during the execution of the program.
/// Lazy ensures that the initialization is thread-safe.
///  If multiple threads try to access UNKNOWN_LOCATION at the same time, Lazy guarantees that the initialization happens only once.
/// The `Box<Location>` involves heap allocation.
///  Using Lazy ensures that this allocation happens only when necessary, avoiding unnecessary memory usage if the constant is never used.
/// The initialization of `UNKNOWN_LOCATION` involves creating a `Location` struct with specific values.
///  Lazy allows you to encapsulate this initialization logic in a closure, making the code cleaner and more maintainable.
///
/// `one_cell` is coming into the standard Rust library (it is already in the nightly build).
///
/// `static` keyword: https://doc.rust-lang.org/std/keyword.static.html
pub(crate) static UNKNOWN_LOCATION: Lazy<Box<Location>> = Lazy::new(|| {
    Box::new(
        Location::new(
            999,
            "UNKNOWN".to_string(),
            1,
            Some("lw-unknown-999".to_string()),
        )
        .unwrap(),
    )
});

/// Location of the Labware
#[derive(Debug, PartialEq, sqlx::FromRow)]
pub struct Location {
    /// ID of the location record
    pub id: u32,
    /// Name of the location
    pub name: String,
    /// The barcode of the location
    pub barcode: Option<String>,
    /// The id of the location_type
    location_type_id: u32,
}

/// Implementation of the Location struct
impl<'a> Location {
    /// Create a new Location
    /// # Examples
    ///
    /// ```
    /// # #[cfg(doctest)] {
    /// use location::Location;
    /// let location_result_1 = Location::new(1, "Building".to_string()); // Returns a Result
    /// let location = location_result_1.unwrap();    // Call unwrap or use match with Err and Ok branches.
    ///
    /// let location_result_2 = Location::new(1, "Building".to_string()); // Returns a Result
    /// let location = match location_result_2 {
    ///     // References on Panic or not to panic?
    ///     // https://doc.rust-lang.org/beta/book/ch09-03-to-panic-or-not-to-panic.html
    ///     Err(e) => panic!("{:?}", e),
    ///     Ok(result) => result
    /// };
    /// # }
    fn new(
        id: u32,
        name: String,
        location_type_id: u32,
        barcode: Option<String>,
    ) -> Result<Location, NameFormatError> {
        if !Location::validate_name(name.clone()) {
            return Err(NameFormatError {
                message: "Invalid name format".to_string(),
            });
        }
        let location = Location {
            id,
            name: name.clone(),
            barcode,
            location_type_id,
        };
        Ok(location)
    }

    /// Create a new Location
    /// # Examples
    /// ```
    /// # #[cfg(doctest)] {
    /// use location::Location;
    /// let location = Location::create("location1".to_string(), 1).await.unwrap();
    /// # }
    /// ```
    pub(crate) async fn create(
        name: String,
        location_type_id: u32,
        connection: &mut SqliteConnection,
    ) -> Result<Location, sqlx::Error> {
        let insert_query_result =
            sqlx::query("INSERT INTO locations (name, location_type_id) VALUES (?, ?)")
                .bind(name.clone())
                .bind(location_type_id)
                .execute(&mut *connection)
                .await?;
        let id = insert_query_result.last_insert_rowid();

        let mut location = Location::new(id as u32, name.clone(), location_type_id, None).unwrap();
        let barcode = location.create_barcode();

        // Catch errors (if any) and handle
        sqlx::query("UPDATE locations SET barcode = ? WHERE id = ?")
            .bind(barcode)
            .bind(id)
            .execute(&mut *connection)
            .await?;

        Ok(location)
    }

    /// Find a location by barcode
    /// # Examples
    /// ```
    /// # #[cfg(doctest)] {
    /// use location::Location;
    /// let mut connection = init_db("sqlite::memory:").await.unwrap();
    /// let location = Location::find_by_barode("lw-location1-1".to_string(), &mut connection).await.unwrap();
    /// # }
    /// ```
    pub(crate) async fn find_by_barcode(
        barcode: String,
        connection: &mut SqliteConnection,
    ) -> Result<Location, NotFoundError> {
        match sqlx::query_as::<_, Location>("SELECT * FROM locations WHERE barcode = ?")
            .bind(barcode)
            .fetch_one(&mut *connection)
            .await
        {
            Ok(location) => Ok(location),
            Err(_) => Err(NotFoundError {
                message: "Location not found".to_string(),
            }),
        }
    }

    /// Create a new unknown location
    /// # Examples
    /// ```
    /// # #[cfg(doctest)] {
    /// use location::Location;
    /// let location = Location::unknown();
    /// # }
    /// ```
    ///
    pub fn unknown() -> &'a Location {
        UNKNOWN_LOCATION.as_ref()
    }

    /// Creates a barcode
    /// Barcode format: `lw-{name trimmed and spaces replaced with "-"}-{id}`
    fn create_barcode(&mut self) -> String {
        let barcode = format!(
            "lw-{}-{}",
            self.name.trim().replace(" ", "-").to_lowercase(),
            self.id
        );
        self.barcode = Some(barcode.clone());
        barcode
    }

    /// Validate the name of the location for a certain format
    /// Validations:
    ///     1. Name must be between 1 and 60 characters
    ///     2. Name must only contain alphanumeric characters, hyphens, spaces, and parentheses
    fn validate_name(name: String) -> bool {
        if !(1..=60).contains(&name.len()) {
            return false;
        }
        Regex::new(r"\A[\w\-\s()]+\z").unwrap().is_match(&name)
    }
}

impl Default for Location {
    fn default() -> Location {
        Location {
            id: 1,
            name: "Location1".to_string(),
            barcode: None,
            location_type_id: 1,
        }
    }
}

/// Error struct for containing name formatting errors
struct NameFormatError {
    /// Message contained within the exception
    message: String,
}

impl Display for NameFormatError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message.to_string())
    }
}

impl Debug for NameFormatError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message.to_string())
    }
}

impl Error for NameFormatError {}

#[cfg(test)]
mod tests {
    use crate::db::init_db;
    use crate::models::location::*;
    use crate::models::location_type::LocationType;

    #[test]
    fn test_location_new() {
        let location = Location::new(
            1,
            "location 1".to_string(),
            1,
            Some("lw-location-1-1".to_string()),
        );
        let location = location.unwrap();
        assert_eq!(location.id, 1);
        assert_eq!(location.name, "location 1");
        assert_eq!(location.barcode.clone().unwrap(), "lw-location-1-1");
        assert_eq!(location.location_type_id, 1);
    }

    #[test]
    fn test_location_names() {
        assert!(Location::new(1, "location 1".to_string(), 1, None).is_ok());
        assert!(Location::new(1, "location one".to_string(), 1, None).is_ok());
        assert!(Location::new(1, "location-one".to_string(), 1, None).is_ok());
        assert!(Location::new(1, "location-one one".to_string(), 1, None).is_ok());
        assert!(Location::new(1, "(A location)".to_string(), 1, None).is_ok());

        assert!(Location::new(1, "A location +++".to_string(), 1, None).is_err());
        assert!(Location::new(1, "A/location".to_string(), 1, None).is_err());
        assert!(Location::new(1, "A location ~".to_string(), 1, None).is_err());
    }

    #[test]
    fn test_location_name_length() {
        assert!(Location::new(1, "".to_string(), 1, None).is_err());
        assert!(Location::new(1, "a".repeat(59), 1, None).is_ok());
        assert!(Location::new(1, "a".repeat(60), 1, None).is_ok());
        assert!(Location::new(1, "a".repeat(61), 1, None).is_err());
    }

    #[test]
    fn test_barcode_sanitisation() {
        let mut location = Location::new(1, "location1".to_string(), 1, None).unwrap();
        location.create_barcode();

        assert_eq!("lw-location1-1", location.barcode.unwrap());

        location = Location::new(1, "location 1".to_string(), 1, None).unwrap();
        location.create_barcode();

        assert_eq!("lw-location-1-1", location.barcode.unwrap());

        location = Location::new(1, "Location1".to_string(), 1, None).unwrap();
        location.create_barcode();

        assert_eq!("lw-location1-1", location.barcode.unwrap());
    }

    #[test]
    fn test_unknown_location() {
        let location = Location::unknown();
        assert_eq!(location.id, 999);
        assert_eq!(location.name, "UNKNOWN");
        assert_eq!(location.barcode.clone().unwrap(), "lw-unknown-999");
    }

    #[tokio::test]
    async fn test_create_location() {
        let mut conn = init_db("sqlite::memory:").await.unwrap();
        let location_type = LocationType::create("Freezer".to_string(), &mut conn)
            .await
            .unwrap();
        let location = Location::create("location1".to_string(), location_type.id, &mut conn)
            .await
            .unwrap();
        assert_eq!(location.name, "location1");
        assert_eq!(location.id, 1);
        assert_eq!(location_type.id, location.location_type_id);
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
        let found_location =
            Location::find_by_barcode(location.barcode.clone().unwrap(), &mut conn)
                .await
                .unwrap();

        assert_eq!(location.barcode, found_location.barcode);
    }

    #[tokio::test]
    async fn test_find_by_barcode_for_not_found() {
        let mut conn = init_db("sqlite::memory:").await.unwrap();
        Location::find_by_barcode("lw-location-1".to_string(), &mut conn)
            .await
            .expect_err("Location not found");
    }
}
