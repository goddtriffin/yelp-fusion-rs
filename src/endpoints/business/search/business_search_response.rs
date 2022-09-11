use crate::models::{Business, Region};
use serde::{Deserialize, Serialize};

/// Response body from Business Search request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessSearchResponse {
    /// Total number of business Yelp finds based on the search criteria.
    ///
    /// Sometimes, the value may exceed 1000.
    /// In such case, you still can only get up to 1000 businesses using multiple queries and
    /// combinations of the "limit" and "offset" parameters.
    pub total: usize,

    /// List of business Yelp finds based on the search criteria.
    pub businesses: Vec<Business>,

    /// Suggested area in a map to display results in.
    pub region: Option<Region>,
}
