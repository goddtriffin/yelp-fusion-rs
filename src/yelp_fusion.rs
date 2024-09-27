use reqwest::blocking::Client as BlockingClient;
use reqwest::Client;

pub const BASE_URL: &str = "https://api.yelp.com/v3";

#[derive(Debug, Clone)]
#[expect(clippy::module_name_repetitions)]
pub struct YelpFusionClient {
    pub(crate) client: Client,
    pub(crate) base_url: String,
    pub(crate) api_key: String,
}

impl YelpFusionClient {
    #[must_use]
    pub fn new(api_key: String, client: Option<Client>) -> Self {
        Self {
            client: client.unwrap_or_default(),
            base_url: BASE_URL.to_string(),
            api_key,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BlockingYelpFusionClient {
    pub(crate) client: BlockingClient,
    pub(crate) base_url: String,
    pub(crate) api_key: String,
}

impl BlockingYelpFusionClient {
    #[must_use]
    pub fn new(api_key: String, client: Option<BlockingClient>) -> Self {
        Self {
            client: client.unwrap_or_default(),
            base_url: BASE_URL.to_string(),
            api_key,
        }
    }
}
