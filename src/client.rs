use crate::error::GraphQLError;
use reqwest::{
  header::{HeaderMap, HeaderName, HeaderValue},
  Client,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

pub struct GQLClient {
  endpoint: &'static str,
  header_map: HeaderMap,
}

#[derive(Serialize)]
struct RequestBody<T: Serialize> {
  query: &'static str,
  variables: Option<T>,
}

#[derive(Deserialize, Debug)]
struct GraphQLResponse<T> {
  pub data: T,
}

impl GQLClient {
  pub fn new(endpoint: &'static str) -> Self {
    Self {
      endpoint,
      header_map: HeaderMap::new(),
    }
  }

  pub fn new_with_headers(endpoint: &'static str, headers: HashMap<&str, &str>) -> Self {
    let mut header_map = HeaderMap::new();

    for (str_key, str_value) in headers {
      let key = HeaderName::from_str(str_key).unwrap();
      let val = HeaderValue::from_str(str_value).unwrap();

      header_map.insert(key, val);
    }

    Self {
      endpoint,
      header_map,
    }
  }

  pub async fn query<K>(&self, query: &'static str) -> Result<K, GraphQLError>
  where
    K: for<'de> Deserialize<'de>,
  {
    self.query_with_vars::<K, ()>(query, ()).await
  }

  pub async fn query_with_vars<K, T: Serialize>(
    &self,
    query: &'static str,
    variables: T,
  ) -> Result<K, GraphQLError>
  where
    K: for<'de> Deserialize<'de>,
  {
    let client = Client::new();
    let body = RequestBody {
      query,
      variables: Some(variables),
    };

    let response = client
      .post(self.endpoint)
      .json(&body)
      .headers(self.header_map.clone())
      .send()
      .await?
      .json::<GraphQLResponse<K>>()
      .await?;

    Ok(response.data)
  }
}
