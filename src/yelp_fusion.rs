use reqwest::Client;

pub const BASE_URL: &str = "https://api.yelp.com/v3";

#[derive(Debug, Clone)]
pub struct YelpFusion {
    pub(crate) client: Client,
    pub(crate) base_url: String,
    pub(crate) api_key: String,
}

impl YelpFusion {
    #[must_use]
    pub fn new(api_key: String, client: Option<Client>) -> Self {
        Self {
            client: client.unwrap_or_else(Client::new),
            base_url: BASE_URL.to_string(),
            api_key,
        }
    }
}
