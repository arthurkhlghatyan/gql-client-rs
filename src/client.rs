use reqwest::{
  header::{HeaderMap, HeaderName, HeaderValue},
  Client, Error,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use std::borrow::Borrow;

pub struct GQLClient {
  endpoint: String,
  header_map: HeaderMap,
}

#[derive(Serialize)]
struct RequestBody<T: Serialize> {
  query: String,
  variables: T,
}

#[derive(Deserialize, Debug)]
struct GraphQLResponse<T> {
  pub data: T,
}

impl GQLClient {
  pub fn new(endpoint: String) -> Self {
    Self { endpoint, header_map: HeaderMap::new() }
  }

  pub fn new_with_headers(endpoint: String, headers: HashMap<String, String>) -> Self {
    let mut header_map = HeaderMap::new();

    for (key, value) in headers {
      header_map.insert(
        HeaderName::from_str(&key).unwrap(),
        HeaderValue::from_str(&value).unwrap(),
      );
    }

    Self {
      endpoint,
      header_map,
    }
  }

  pub async fn query<T: Serialize, K>(
    &self,
    query: String,
    variables: T,
  ) -> Result<K, Error>
  where
    K: for<'de> Deserialize<'de>,
  {
    let client = Client::new();
    let body = RequestBody { query, variables };

    let response = client
      .post(&self.endpoint)
      .json(&body)
      .headers(self.header_map.clone())
      .send()
      .await?
      .json::<GraphQLResponse<K>>()
      .await?;

    Ok(response.data)
  }
}
