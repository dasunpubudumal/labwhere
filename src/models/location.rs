use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use regex::Regex;

/// Location of the Labware
#[derive(Debug)]
struct Location {
    /// ID of the location record
    id: u32,
    /// Name of the location
    name: String,
    /// The barcode of the location
    barcode: String
}

/// Implementation of the Location struct
impl Location {
    /// Create a new Location
    /// # Examples
    ///
    /// ```
    /// use location::Location;
    /// let location = Location::new(1, "Building".to_string(),
    fn new(id: u32, name: String) -> Result<Location, NameFormatError> {
        if !Location::validate_name(name.clone()) {
            return Err(NameFormatError { message: "Invalid name format".to_string() });
        }
        Ok(Location { id, name, barcode: "".to_string() })
    }

    /// Validate the name of the location for a certain format
    fn validate_name(name: String) -> bool {
        if name.len() == 0 {
            return false
        }
        if name.len() > 60 {
            return false
        }
        let re = Regex::new(r"\A[\w\-\s()]+\z").unwrap();
        if re.is_match(&name) {
            return true;
        }
        false
    }
}

struct NameFormatError {
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
        assert_eq!(location.barcode, "");
    }

    #[test]
    fn test_location_names() {
        assert!(Location::new(1, "location 1".to_string()).is_ok());
        assert!(Location::new(1, "location one".to_string()).is_ok());
        assert!(Location::new(1, "location-one".to_string()).is_ok());
        assert!(Location::new(1, "location-one one".to_string()).is_ok());

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

}