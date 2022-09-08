use crate::error::{ApiErrorResponse, Error};
use crate::models::{Attribute, Business, Coordinates, PriceType, Region, SortBy};
use crate::yelp_fusion::YelpFusion;
use bytes::Bytes;
use reqwest::{RequestBuilder, StatusCode};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

impl YelpFusion {
    /// # Errors
    ///
    /// Will return `Err` if an error occurred while creating/sending the request,
    /// if it failed to decode the response's bytes, if the response's status code was not a
    /// success, or if it failed to serialize the response bytes into `BusinessSearchResponse`.
    pub async fn business_search(
        self,
        payload: BusinessSearchPayload,
    ) -> Result<BusinessSearchResponse, Error> {
        // create request
        let request: RequestBuilder = self
            .client
            .get(format!("{}/businesses/search", self.base_url))
            .bearer_auth(self.api_key)
            .query(&payload.to_query_params());

        // send request, get response
        let response = request.send().await?;

        // parse status code and returned bytes
        let status_code: StatusCode = response.status();
        let bytes: Bytes = response.bytes().await?;

        // check if failure
        if !status_code.is_success() {
            let error_response: ApiErrorResponse = serde_json::from_slice(&bytes)?;
            return Err(Error::RequestFailed {
                error_response,
                status_code,
            });
        }

        // success
        let response: BusinessSearchResponse = serde_json::from_slice(&bytes)?;
        Ok(response)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessSearchPayload {
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
    /// If the specified value is too large, a AREA_TOO_LARGE error may be returned.
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
    /// best_match, rating, review_count or distance.
    ///
    /// The default is best_match.
    /// Note that specifying the sort_by is a suggestion (not strictly enforced) to Yelp's search,
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
    /// Notice that open_at and open_now cannot be used together.
    pub open_now: Option<bool>,

    /// An integer representing the Unix time in the same timezone of the search location.
    ///
    /// If specified, it will return business open at the given time.
    ///
    /// Notice that open_at and open_now cannot be used together.
    pub open_at: Option<usize>,

    /// Try these additional filters to return specific search results!
    ///
    /// You can combine multiple attributes by providing a comma separated like "attribute1,attribute2".
    /// If multiple attributes are used, only businesses that satisfy ALL attributes will be returned
    /// in search results.
    /// For example, the attributes "hot_and_new,request_a_quote" will return businesses that are Hot
    /// and New AND offer Request a Quote.
    pub attributes: Option<HashSet<Attribute>>,
}

impl BusinessSearchPayload {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        term: Option<String>,
        location: Option<String>,
        coordinates: Option<Coordinates>,
        radius: Option<usize>,
        categories: Option<Vec<String>>,
        locale: Option<String>,
        limit: Option<usize>,
        offset: Option<usize>,
        sort_by: Option<SortBy>,
        price: Option<HashSet<PriceType>>,
        open_now: Option<bool>,
        open_at: Option<usize>,
        attributes: Option<HashSet<Attribute>>,
    ) -> Self {
        Self {
            term,
            location,
            coordinates,
            radius,
            categories,
            locale,
            limit,
            offset,
            sort_by,
            price,
            open_now,
            open_at,
            attributes,
        }
    }

    #[must_use]
    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut query_params: Vec<(&str, String)> = vec![];

        // term
        if let Some(term) = &self.term {
            query_params.push(("term", term.clone()));
        }

        // location
        if let Some(location) = &self.location {
            query_params.push(("location", location.clone()));
        }

        // latitude
        if let Some(coordinates) = &self.coordinates {
            query_params.push(("latitude", coordinates.latitude.to_string()));
            query_params.push(("longitude", coordinates.longitude.to_string()));
        }

        // radius
        if let Some(radius) = &self.radius {
            query_params.push(("radius", radius.to_string()));
        }

        // categories
        if let Some(categories) = &self.categories {
            query_params.push(("categories", categories.join(",")));
        }

        // locale
        if let Some(locale) = &self.locale {
            query_params.push(("locale", locale.clone()));
        }

        // limit
        if let Some(limit) = &self.limit {
            query_params.push(("limit", limit.to_string()));
        }

        // offset
        if let Some(offset) = &self.offset {
            query_params.push(("offset", offset.to_string()));
        }

        // sort_by
        if let Some(sort_by) = &self.sort_by {
            query_params.push(("sort_by", sort_by.to_string()));
        }

        // price
        if let Some(price) = &self.price {
            let comma_delimited_prices = price
                .iter()
                .map(|price| price.as_usize().to_string())
                .collect::<Vec<String>>()
                .join(",");
            query_params.push(("price", comma_delimited_prices));
        }

        // open_now
        if let Some(open_now) = &self.open_now {
            query_params.push(("open_now", open_now.to_string()));
        }

        // open_at
        if let Some(open_at) = &self.open_at {
            query_params.push(("open_at", open_at.to_string()));
        }

        // attributes
        if let Some(attributes) = &self.attributes {
            let comma_delimited_attributes = attributes
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>()
                .join(",");
            query_params.push(("attributes", comma_delimited_attributes));
        }

        query_params
    }
}

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
