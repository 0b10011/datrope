use std::collections::HashMap;

use reqwest::{Client, Method, Response};
use url::Url;

use super::Result;

pub struct HttpClient {
    base_url: Url,
    base_query: Vec<(&'static str, &'static str)>,
    client: Client,
}

impl HttpClient {
    pub fn new(base_url: Url) -> Result<HttpClient> {
        Ok(HttpClient {
            base_url,
            base_query: vec![],
            client: reqwest::Client::builder().build()?,
        })
    }

    pub async fn request(
        &self,
        method: Method,
        api_path: &str,
        query: &HashMap<&str, &str>,
    ) -> Result<Response> {
        let url = self.base_url.join(api_path.trim_start_matches('/'))?;

        Ok(self
            .client
            .request(method, url)
            .query(query)
            .query(&self.base_query)
            .send()
            .await?)
    }

    pub async fn get(&self, api_path: &str, query: &HashMap<&str, &str>) -> Result<Response> {
        self.request(Method::GET, api_path, query).await
    }
}
