use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// Suggestion to the search algorithm that the results be sorted.
///
/// The default is `best_match`.
/// Note that specifying the `sort_by` is a suggestion (not strictly enforced) to Yelp's search,
/// which considers multiple input parameters to return the most relevant results.
///
/// For example, the rating sort is not strictly sorted by the rating value, but by an adjusted
/// rating value that takes into account the number of ratings, similar to a Bayesian average.
/// This is to prevent skewing results to businesses with a single review.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortBy {
    BestMatch,
    Rating,
    ReviewCount,
    Distance,
}

impl Default for SortBy {
    fn default() -> Self {
        Self::BestMatch
    }
}

impl SortBy {
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::BestMatch => "best_match",
            Self::Rating => "rating",
            Self::ReviewCount => "review_count",
            Self::Distance => "distance",
        }
    }
}

impl Display for SortBy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
