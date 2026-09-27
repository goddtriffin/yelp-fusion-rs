use crate::models::Coordinates;
use serde::{Deserialize, Serialize};

/// Suggested area in a map to display results in.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Region {
    pub center: Coordinates,
}
