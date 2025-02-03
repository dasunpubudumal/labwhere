use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
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

    pub fn create(value: Value) -> Result<Scan, ()> {
        Ok(Scan {
            labware_barcode: "".to_string(),
            location_barcode: "".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::models::scan::Scan;

    #[test]
    fn test_scan_new() {
        let scan = Scan::new("lw-bc-1".to_string(), "lc-bc-1".to_string());
        assert_eq!(scan.location_barcode, "lc-bc-1".to_string());
        assert_eq!(scan.labware_barcode, "lw-bc-1".to_string());
    }
}
