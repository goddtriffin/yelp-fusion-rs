use crate::endpoints::{BusinessSearchPayload, BusinessSearchPayloadError};
use crate::models::{Attribute, Coordinates, PriceType, SortBy};
use std::collections::HashSet;

#[derive(Debug, Clone, Default)]
pub struct BusinessSearchPayloadBuilder {
    /// Search term, for example "food" or "restaurants".
    ///
    /// The term may also be business names, such as "Starbucks".
    /// If term is not included the endpoint will default to searching across businesses from a small number of popular categories.
    pub term: Option<String>,

    /// Required if either latitude or longitude is not provided.
    ///
    /// This string indicates the geographic area to be used when searching for businesses.
    /// Examples: "New York City", "NYC", "350 5th Ave, New York, NY 10118".
    ///
    /// Businesses returned in the response may not be strictly within the specified location.
    pub location: Option<String>,

    /// Required if location is not provided.
    ///
    /// Latitude and longitude of the location you want to search nearby.
    pub coordinates: Option<Coordinates>,

    /// A suggested search radius in meters.
    ///
    /// This field is used as a suggestion to the search.
    /// The actual search radius may be lower than the suggested radius in dense urban areas, and higher in regions of less business density.
    /// If the specified value is too large, a `AREA_TOO_LARGE` error may be returned.
    /// The max value is 40000 meters (about 25 miles).
    pub radius: Option<usize>,

    /// Categories to filter the search results with.
    ///
    /// See the list of [supported categories](https://www.yelp.com/developers/documentation/v3/all_category_list).
    /// The category filter can be a list of comma delimited categories.
    /// For example, "bars,french" will filter by Bars OR French.
    /// The category identifier should be used (for example "discgolf", not "Disc Golf").
    pub categories: Option<Vec<String>>,

    /// Specify the locale into which to localize the business information.
    ///
    /// See the list of [supported locales](https://www.yelp.com/developers/documentation/v3/supported_locales).
    /// Defaults to `"en_US"`.
    pub locale: Option<String>,

    /// Number of business results to return.
    ///
    /// By default, it will return 20.
    /// Maximum is 50.
    pub limit: Option<usize>,

    /// Offset the list of returned business results by this amount.
    pub offset: Option<usize>,

    /// Suggestion to the search algorithm that the results be sorted by one of the these modes:
    /// `best_match`, rating, `review_count` or distance.
    ///
    /// The default is `best_match`.
    /// Note that specifying the `sort_by` is a suggestion (not strictly enforced) to Yelp's search,
    /// which considers multiple input parameters to return the most relevant results.
    ///
    /// For example, the rating sort is not strictly sorted by the rating value, but by an adjusted
    /// rating value that takes into account the number of ratings, similar to a Bayesian average.
    /// This is to prevent skewing results to businesses with a single review.
    pub sort_by: Option<SortBy>,

    /// Pricing levels to filter the search result with: 1 = $, 2 = $$, 3 = $$$, 4 = $$$$.
    ///
    /// The price filter can be a list of comma delimited pricing levels.
    /// For example, "1, 2, 3" will filter the results to show the ones that are $, $$, or $$$.
    pub price: Option<HashSet<PriceType>>,

    /// Default to false.
    /// When set to true, only return the businesses open now.
    ///
    /// Notice that `open_at` and `open_now` cannot be used together.
    pub open_now: Option<bool>,

    /// An integer representing the Unix time in the same timezone of the search location.
    ///
    /// If specified, it will return business open at the given time.
    ///
    /// Notice that `open_at` and `open_now` cannot be used together.
    pub open_at: Option<usize>,

    /// Try these additional filters to return specific search results!
    ///
    /// You can combine multiple attributes by providing a comma separated like "attribute1,attribute2".
    /// If multiple attributes are used, only businesses that satisfy ALL attributes will be returned
    /// in search results.
    /// For example, the attributes "`hot_and_new,request_a_quote`" will return businesses that are Hot
    /// and New AND offer Request a Quote.
    pub attributes: Option<HashSet<Attribute>>,
}

impl BusinessSearchPayloadBuilder {
    pub fn term(&mut self, term: String) -> &mut Self {
        self.term = Some(term);
        self
    }

    pub fn location(&mut self, location: String) -> &mut Self {
        self.location = Some(location);
        self
    }

    pub fn coordinates(&mut self, coordinates: Coordinates) -> &mut Self {
        self.coordinates = Some(coordinates);
        self
    }

    pub fn radius(&mut self, radius: usize) -> &mut Self {
        self.radius = Some(radius);
        self
    }

    pub fn categories(&mut self, categories: Vec<String>) -> &mut Self {
        self.categories = Some(categories);
        self
    }

    pub fn locale(&mut self, locale: String) -> &mut Self {
        self.locale = Some(locale);
        self
    }

    pub fn limit(&mut self, limit: usize) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(&mut self, offset: usize) -> &mut Self {
        self.offset = Some(offset);
        self
    }

    pub fn sort_by(&mut self, sort_by: SortBy) -> &mut Self {
        self.sort_by = Some(sort_by);
        self
    }

    pub fn price(&mut self, price: HashSet<PriceType>) -> &mut Self {
        self.price = Some(price);
        self
    }

    pub fn open_now(&mut self, open_now: bool) -> &mut Self {
        self.open_now = Some(open_now);
        self
    }

    pub fn open_at(&mut self, open_at: usize) -> &mut Self {
        self.open_at = Some(open_at);
        self
    }

    pub fn attributes(&mut self, attributes: HashSet<Attribute>) -> &mut Self {
        self.attributes = Some(attributes);
        self
    }

    /// # Errors
    ///
    /// Returns `BusinessSearchPayloadError::BothLocationAndLatLongSet` if both `location` and
    /// `coordinates` are set. Only one or the other can be set.
    ///
    /// Returns `BusinessSearchPayloadError::RadiusTooLarge` if `radius` is over `40,000` meters
    /// (~25 miles).
    ///
    /// Returns `BusinessSearchPayloadError::LimitTooLarge` if `limit` is over `50`.
    ///
    /// Returns `BusinessSearchPayloadError::BothOpenNowAndOpenAtSet` if both `open_now` and
    /// `open_at` are set. Only one of the other can be set.
    pub fn build(&self) -> Result<BusinessSearchPayload, BusinessSearchPayloadError> {
        BusinessSearchPayload::new(
            self.term.clone(),
            self.location.clone(),
            self.coordinates,
            self.radius,
            self.categories.clone(),
            self.locale.clone(),
            self.limit,
            self.offset,
            self.sort_by,
            self.price.clone(),
            self.open_now,
            self.open_at,
            self.attributes.clone(),
        )
    }
}
