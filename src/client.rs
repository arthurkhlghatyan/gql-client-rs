use std::{collections::HashMap, str::FromStr, time::Duration};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

#[cfg(feature = "blocking")]
use reqwest::blocking::Client;
#[cfg(not(feature = "blocking"))]
use reqwest::Client;

use serde::{Deserialize, Serialize};

use crate::error::{GraphQLError, GraphQLErrorMessage};

#[derive(Clone, Debug)]
pub struct GQLClient <'a> {
  endpoint: &'a str,
  client: Client,
}

#[derive(Serialize)]
struct RequestBody<'a, T: Serialize> {
  query: &'a str,
  variables: T,
}

#[derive(Deserialize, Debug)]
struct GraphQLResponse<T> {
  data: Option<T>,
  errors: Option<Vec<GraphQLErrorMessage>>,
}

impl <'a> GQLClient <'a> {
  pub fn new(endpoint: &'a str) -> Self {
    Self {
      endpoint: &endpoint,
      client: if cfg!(target_arch = "wasm32") {
        Client::new()
      } else {
        Client::builder()
          .timeout(Duration::from_secs(5))
          .build()
          .unwrap()
      },
    }
  }

  pub fn new_with_headers(endpoint: &'a str, headers: HashMap<&str, &str>) -> Self {
    let mut header_map = HeaderMap::new();

    for (str_key, str_value) in headers {
      let key = HeaderName::from_str(str_key).unwrap();
      let val = HeaderValue::from_str(str_value).unwrap();

      header_map.insert(key, val);
    }

    Self {
      endpoint: &endpoint,
      client: if cfg!(target_arch = "wasm32") {
        Client::builder().default_headers(header_map).build().unwrap()
      } else {
        Client::builder()
          .timeout(Duration::from_secs(5))
          .default_headers(header_map)
          .build()
          .unwrap()
      },
    }
  }

 #[cfg(not(feature = "blocking"))]
 pub async fn query<K>(&self, query: &'a str) -> Result<K, GraphQLError>
  where
    K: for<'de> Deserialize<'de>,
  {
    self.query_with_vars::<K, ()>(query, ()).await
  }

 #[cfg(not(feature = "blocking"))]
  pub async fn query_with_vars<K, T: Serialize>(
    &self,
    query: &'a str,
    variables: T,
  ) -> Result<K, GraphQLError>
  where
    K: for<'de> Deserialize<'de>,
  {
    let body = RequestBody { query, variables };

    let request = self.client
      .post(self.endpoint)
      .json(&body);

    let raw_response = request.send().await?;
    let json_response = raw_response.json::<GraphQLResponse<K>>().await;

    // Check whether JSON is parsed successfully
    match json_response {
      Ok(json) => {
        // Check if error messages have been received
        if json.errors.is_some() {
          return Err(GraphQLError::from_json(json.errors.unwrap()));
        }

        Ok(json.data.unwrap())
      }
      Err(_e) => Err(GraphQLError::from_str("Failed to parse response")),
    }
  }

#[cfg(feature = "blocking")]
 pub fn query<K>(&self, query: &'a str) -> Result<K, GraphQLError>
  where
    K: for<'de> Deserialize<'de>,
  {
    self.query_with_vars::<K, ()>(query, ())
  }

 #[cfg(feature = "blocking")]
  pub fn query_with_vars<K, T: Serialize>(
    &self,
    query: &'a str,
    variables: T,
  ) -> Result<K, GraphQLError>
  where
    K: for<'de> Deserialize<'de>,
  {
    let client = Client::new();
    let body = RequestBody { query, variables };

    let request = client
      .post(self.endpoint)
      .json(&body);

    let raw_response = request.send()?;
    let json_response = raw_response.json::<GraphQLResponse<K>>();

    // Check whether JSON is parsed successfully
    match json_response {
      Ok(json) => {
        // Check if error messages have been received
        if json.errors.is_some() {
          return Err(GraphQLError::from_json(json.errors.unwrap()));
        }

        Ok(json.data.unwrap())
      }
      Err(_e) => Err(GraphQLError::from_str("Failed to parse response")),
    }
  }
}
