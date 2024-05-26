use std::rc::Rc;

use reqwest::StatusCode;
use thiserror::Error;
use url::Url;

use crate::api::client::endpoints::gateway::Gateway;

use self::http_client::HttpClient;

mod http_client;

pub mod endpoints;

pub type Result<T> = std::result::Result<T, ApiError>;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Failed to build the reqwest client")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Failed to parse a URL")]
    UrlParseError(#[from] url::ParseError),
    #[error("Received an unexpected response from the API (HTTP {status_code}): {response}")]
    UnexpectedResponse {
        status_code: StatusCode,
        response: String,
    },
}

pub struct Api {
    gateway: Gateway,
}

impl Api {
    pub fn new(base_url: Url) -> Result<Api> {
        let http_client = Rc::new(HttpClient::new(base_url)?);
        Ok(Api {
            gateway: Gateway::new(http_client),
        })
    }

    pub fn gateway(&self) -> &Gateway {
        &self.gateway
    }
}
