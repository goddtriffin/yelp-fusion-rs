use serde::{Deserialize, Serialize};

/// The coordinates of this business.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Coordinates {
    /// Latitude of this business.
    pub latitude: f32,

    /// Longitude of this business.
    pub longitude: f32,
}

impl Coordinates {
    #[must_use]
    pub const fn new(latitude: f32, longitude: f32) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}
