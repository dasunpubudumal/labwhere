use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use regex::Regex;
use crate::models::location_type::LocationType;

/// Location of the Labware
#[derive(Debug)]
struct Location {
    /// ID of the location record
    id: u32,
    /// Name of the location
    name: String,
    /// The barcode of the location
    barcode: String,
    /// The type of location
    location_type: LocationType
}

/// Implementation of the Location struct
impl Location {
    /// Create a new Location
    /// # Examples
    ///
    /// ```
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
    fn new(id: u32, name: String) -> Result<Location, NameFormatError> {
        if !Location::validate_name(name.clone()) {
            return Err(NameFormatError { message: "Invalid name format".to_string() });
        }
        let location = Location {
            id,
            name: name.clone(),
            barcode: Location::create_barcode(&name, &id),
            location_type: Default::default()
        };
        Ok(location)
    }

    /// Creates a barcode
    /// Barcode format: `lw-{name trimmed and spaces replaced with "-"}-{id}`
    fn create_barcode(name: &String, id: &u32) -> String {
        return format!("lw-{}-{}", name.clone().trim().replace(" ", "-").to_lowercase(), id.clone());
    }

    /// Validate the name of the location for a certain format
    /// Validations:
    ///     1. Name must be between 1 and 60 characters
    ///     2. Name must only contain alphanumeric characters, hyphens, spaces, and parentheses
    fn validate_name(name: String) -> bool {
        if !(1..=60).contains(&name.len()) {
            return false
        }
        Regex::new(r"\A[\w\-\s()]+\z").unwrap().is_match(&name)
    }
}

/// Error struct for containing name formatting errors
struct NameFormatError {
    /// Message contained within the exception
    message: String
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

    use super::*;

    #[test]
    fn test_location_new() {
        let location = Location::new(1, "location 1".to_string());
        let location = location.unwrap();
        assert_eq!(location.id, 1);
        assert_eq!(location.name, "location 1");
        assert_eq!(location.barcode, "lw-location-1-1");
        assert_eq!(location.location_type.id, 1);
    }

    #[test]
    fn test_location_names() {
        assert!(Location::new(1, "location 1".to_string()).is_ok());
        assert!(Location::new(1, "location one".to_string()).is_ok());
        assert!(Location::new(1, "location-one".to_string()).is_ok());
        assert!(Location::new(1, "location-one one".to_string()).is_ok());
        assert!(Location::new(1, "(A location)".to_string()).is_ok());

        assert!(Location::new(1, "A location +++".to_string()).is_err());
        assert!(Location::new(1, "A/location".to_string()).is_err());
        assert!(Location::new(1, "A location ~".to_string()).is_err());
    }

    #[test]
    fn test_location_name_length() {
        assert!(Location::new(1,"".to_string()).is_err());
        assert!(Location::new(1,"a".repeat(59)).is_ok());
        assert!(Location::new(1,"a".repeat(60)).is_ok());
        assert!(Location::new(1, "a".repeat(61)).is_err());
    }

    #[test]
    fn test_barcode_sanitisation() {
        assert_eq!("lw-location1-1", Location::new(1, "location1".to_string()).unwrap().barcode);
        assert_eq!("lw-location-1-1", Location::new(1, "location 1".to_string()).unwrap().barcode);
        assert_eq!("lw-location1-1", Location::new(1, "Location1".to_string()).unwrap().barcode);
    }
}
