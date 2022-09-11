use std::error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum BusinessSearchPayloadError {
    /// Returned when both Location and Latitude/Longitude are set (must use either-or).
    BothLocationAndLatLongSet,

    /// Returned when `radius` is over `40,000` meters (approx. `25` miles).
    RadiusTooLarge(usize),

    /// Returned when `limit` is set over `50`.
    LimitTooLarge(usize),

    /// Returned when `open_now` and `open_at` are set (must use either-or).
    BothOpenNowAndOpenAtSet,
}

impl error::Error for BusinessSearchPayloadError {}

impl Display for BusinessSearchPayloadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BothLocationAndLatLongSet => {
                write!(f, "can either set location OR latitude/longitude, NOT both")
            }
            Self::RadiusTooLarge(radius) => {
                write!(f, "radius must not be above 40,000 meters: {}", radius)
            }
            Self::LimitTooLarge(limit) => write!(f, "limit must not be over 50: {}", limit),
            Self::BothOpenNowAndOpenAtSet => {
                write!(f, "can only set open_now OR open_at, not both")
            }
        }
    }
}
