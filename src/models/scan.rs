use crate::errors::NotFoundError;
use crate::models::location::Location;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteConnection;

#[derive(Serialize, Deserialize, Debug)]
pub struct Scan {
    labware_barcode: String,
    location_barcode: String,
}

impl Scan {
    pub fn new(labware_barcode: String, location_barcode: String) -> Scan {
        Scan {
            labware_barcode,
            location_barcode,
        }
    }

    /// Creates a Scan model after validations.
    ///
    /// 1. Find the location by its barcode `location_barcode`.
    /// 2. If the location doesn't exist, it would err. The service would respond to the client
    /// depending on the error type.
    /// 3. Find the labware by its barcode `labware_barcode`.
    /// 4. If the labware exists, return it. If it doesn't exist, create the labware in the database.
    pub async fn create(
        scan: Scan,
        connection: &mut SqliteConnection,
    ) -> Result<Scan, NotFoundError> {
        // TODO: Complete this function.

        // let location_result = Location::find_by_barcode(scan.location_barcode);
        let location: Location =
            match Location::find_by_barcode(scan.location_barcode, connection).await {
                Ok(location) => location,
                Err(error) => {
                    // Respond with a 404
                    return Err(error);
                }
            };

        Ok(Scan {
            labware_barcode: "".to_string(),
            location_barcode: "".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::db::init_db;
    use crate::models::scan::Scan;

    #[test]
    fn test_scan_new() {
        let scan = Scan::new("lw-bc-1".to_string(), "lc-bc-1".to_string());
        assert_eq!(scan.location_barcode, "lc-bc-1".to_string());
        assert_eq!(scan.labware_barcode, "lw-bc-1".to_string());
    }

    #[tokio::test]
    async fn test_scan_create_no_location() {
        let scan = Scan::new("lw-bc-1".to_string(), "lc-bc-1".to_string());
        let mut connection = init_db("sqlite::memory:").await.unwrap();
        Scan::create(scan, &mut connection)
            .await
            .expect_err("Location not found");
    }
}
