use serde::{Deserialize, Serialize};

/// Category to filter the search results with.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    /// Alias of a category, when searching for business in certain categories, use alias rather than the title.
    pub alias: String,

    /// Title of a category for display purpose.
    pub title: String,
}
