use crate::models::location::Location;

use super::location::UNKNOWN_LOCATION;

/// Labware is stored in a location.
/// LabWhere needs to know nothing about it apart from its barcode and where it is.
/// If a labware has no location it's location will be set to unknown automatically
struct Labware<'a> {
    /// The unique identifier for the Labware
    id: u32,
    /// The unique barcode of the Labware
    barcode: String,
    /// The location of the Labware
    location: &'a Location,
}

/// Implementation of the Labware struct
impl<'a> Labware<'a> {
    /// Create a new Labware
    /// # Examples
    /// ```
    /// use labware::Labware;
    /// let location:Location = Default::default();
    /// let labware = Labware::new(1, "trac-1".to_string(), location);
    /// ```
    ///
    fn new(id: u32, barcode: String, location: Option<&'a Location>) -> Labware {
        Labware {
            id,
            barcode,
            location: location.unwrap_or(&UNKNOWN_LOCATION),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_labware_new() {
        let location: Location = Default::default();
        let labware = Labware::new(1, "lw-1".to_string(), Some(&location));
        assert_eq!(labware.id, 1);
        assert_eq!(labware.barcode, "lw-1");
        assert_eq!(*labware.location, location);
    }

    #[test]
    fn test_labware_no_location() {
        let labware = Labware::new(1, "lw-1".to_string(), None);
        assert_eq!(labware.id, 1);
        assert_eq!(labware.barcode, "lw-1");
        assert_eq!(labware.location.name, "UNKNOWN".to_string());
    }
}
