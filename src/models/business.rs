use crate::models::{Category, Coordinates, Location, PriceType, Region, TransactionType};
use serde::{Deserialize, Serialize};

/// A Yelp business.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Business {
    /// Unique Yelp ID of this business.
    ///
    /// Example: '4kMBvIEWPxWkWKFN__8SxQ'
    pub id: String,

    /// Unique Yelp alias of this business.
    ///
    /// Can contain unicode characters.
    /// Example: 'yelp-san-francisco'.
    pub alias: Option<String>,

    /// Name of this business.
    pub name: Option<String>,

    /// URL of photo for this business.
    pub image_url: Option<String>,

    /// Whether business has been (permanently) closed.
    pub is_closed: bool,

    /// URL for business page on Yelp.
    pub url: Option<String>,

    /// Number of reviews for this business.
    pub review_count: usize,

    /// List of category title and alias pairs associated with this business.
    pub categories: Vec<Category>,

    /// Rating for this business (value ranges from 1, 1.5, ... 4.5, 5).
    pub rating: f32,

    /// Coordinates of this business.
    pub coordinates: Coordinates,

    /// List of Yelp transactions that the business is registered for.
    ///
    /// Current supported values are pickup, delivery and restaurant_reservation.
    pub transactions: Vec<TransactionType>,

    /// Price level of the business.
    ///
    /// Value is one of $, $$, $$$ and $$$$.
    pub price: Option<PriceType>,

    /// Location of this business, including address, city, state, zip code and country.
    pub location: Location,

    /// Phone number of the business.
    pub phone: String,

    /// Phone number of the business formatted nicely to be displayed to users.
    /// The format is the standard phone number format for the business's country.
    pub display_phone: String,

    /// Distance in meters from the search location.
    /// This returns meters regardless of the locale.
    pub distance: f32,

    /// Suggested area in a map to display results in.
    pub region: Option<Region>,
}
