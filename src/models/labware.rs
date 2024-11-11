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
    /// # #[cfg(doctest)] {
    /// use labware::Labware;
    /// let location:Location = Default::default();
    /// let labware = Labware::new(1, "trac-1".to_string(), location);
    /// # }
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
    use crate::models::labware::*;

    #[test]
    fn test_labware_new() {
        let location: Location = Default::default();
        let labware = Labware::new(1, "lw-1".to_string(), Some(&location));
        assert_eq!(labware.id, 1);
        assert_eq!(labware.barcode, "lw-1");
        assert_eq!(*labware.location, location);
    }

    // We should have an equal function for the Labware struct that relies on the attributes of the
    // struct. This will allow us to compare two Labware structs and check if they are equal.
    #[test]
    fn test_labware_no_location() {
        let labware = Labware::new(1, "lw-1".to_string(), None);
        assert_eq!(labware.id, 1);
        assert_eq!(labware.barcode, "lw-1");
        assert_eq!(*labware.location, *UNKNOWN_LOCATION.as_ref());
    }
}
