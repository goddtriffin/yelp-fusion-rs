use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// Additional filters to return specific search results.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Attribute {
    /// [Popular businesses](https://www.yelp.com/search?attrs=NewBusiness&find_desc=Restaurants)
    /// which recently joined Yelp.
    HotAndNew,

    /// Businesses which actively reply to [Request a Quote](https://www.yelpblog.com/2016/04/yelp-request-a-quote)
    /// inquiries.
    RequestAQuote,

    /// Businesses with [Yelp Reservations](https://www.yelp.com/search?find_desc=Reservations) bookings
    /// enabled on their profile page.
    Reservation,

    /// Businesses with [Yelp Waitlist](https://www.yelpblog.com/2016/10/waitlist-times-get-line-remotely-yelp-nowait)
    /// bookings enabled on their profile screen (iOS/Android).
    WaitlistReservation,

    /// Businesses offering [Yelp Deals](https://www.yelp.com/search?find_desc=deals) on their profile page.
    Deals,

    /// Businesses which provide [gender neutral restrooms](https://www.yelpblog.com/2017/03/now-use-yelp-find-gender-neutral-restrooms).
    GenderNeutralRestrooms,

    /// Businesses which are [Open To All](https://www.yelpblog.com/2018/07/opentoall).
    OpenToAll,

    /// Businesses which are [Wheelchair Accessible](https://www.yelp.com/search?attrs=WheelchairAccessible).
    WheelchairAccessible,
}

impl Attribute {
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::HotAndNew => "hot_and_new",
            Self::RequestAQuote => "request_a_quote",
            Self::Reservation => "reservation",
            Self::WaitlistReservation => "waitlist_reservation",
            Self::Deals => "deals",
            Self::GenderNeutralRestrooms => "gender_neutral_restrooms",
            Self::OpenToAll => "open_to_all",
            Self::WheelchairAccessible => "wheelchair_accessible",
        }
    }
}

impl Display for Attribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
