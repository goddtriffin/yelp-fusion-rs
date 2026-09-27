use serde::{Deserialize, Serialize};

/// The location of this business, including address, city, state, zip code and country.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    /// Street address of this business.
    pub address1: String,

    /// Street address of this business, continued.
    pub address2: Option<String>,

    /// Street address of this business, continued.
    pub address3: Option<String>,

    /// City of this business.
    pub city: String,

    /// Zip code of this business.
    pub zip_code: String,

    /// ISO 3166-1 alpha-2 country code of this business.
    pub country: String,

    /// ISO 3166-2 (with a few exceptions) state code of this business.
    pub state: String,

    /// Array of strings that, if organized vertically, give an address that is in the standard address format for the business's country.
    pub display_address: Vec<String>,
}
