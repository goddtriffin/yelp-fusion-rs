mod business_search_payload;
mod business_search_payload_builder;
mod business_search_payload_error;
mod business_search_response;

use crate::error::{ApiErrorResponse, Error};
use crate::yelp_fusion::YelpFusion;
pub use business_search_payload::*;
pub use business_search_payload_builder::*;
pub use business_search_payload_error::*;
pub use business_search_response::*;
use bytes::Bytes;
use reqwest::{RequestBuilder, StatusCode};

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
