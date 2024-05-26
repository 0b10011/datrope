use std::collections::HashMap;
use std::rc::Rc;

use reqwest::StatusCode;

use crate::api::client::http_client::HttpClient;

use crate::api::client::{ApiError, Result};

pub struct Gateway {
    http_client: Rc<HttpClient>,
}

impl Gateway {
    pub fn new(http_client: Rc<HttpClient>) -> Self {
        Gateway { http_client }
    }

    /// https://discord.com/developers/docs/topics/gateway#get-gateway
    pub async fn get_gateway(&self) -> Result<responses::Gateway> {
        let response = self.http_client.get("/gateway", &HashMap::new()).await?;

        match response.status() {
            StatusCode::OK => Ok(response.json::<responses::Gateway>().await?),
            status_code => Err(ApiError::UnexpectedResponse {
                status_code,
                response: response.text().await?,
            }),
        }
    }
}

mod responses {
    use serde::Deserialize;
    use url::Url;

    #[derive(Debug, Deserialize)]
    pub struct Gateway {
        pub url: Url,
    }
}
