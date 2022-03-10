use std::collections::HashMap;
use std::str::FromStr;

use reqwest::{
  header::{HeaderMap, HeaderName, HeaderValue},
  Client,
};
use serde::{Deserialize, Serialize};

use crate::error::{GraphQLError, GraphQLErrorMessage};

#[derive(Clone, Debug)]
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
  data: Option<T>,
  errors: Option<Vec<GraphQLErrorMessage>>,
}

impl GQLClient {
  pub fn new(endpoint: impl AsRef<str>) -> Self {
    Self {
      endpoint: endpoint.as_ref().to_string(),
      header_map: HeaderMap::new(),
    }
  }

  pub fn new_with_headers(endpoint: impl AsRef<str>, headers: HashMap<&str, &str>) -> Self {
    let mut header_map = HeaderMap::new();

    for (str_key, str_value) in headers {
      let key = HeaderName::from_str(str_key).unwrap();
      let val = HeaderValue::from_str(str_value).unwrap();

      header_map.insert(key, val);
    }

    Self {
      endpoint: endpoint.as_ref().to_string(),
      header_map,
    }
  }

  pub async fn query<K>(&self, query: &str) -> Result<Option<K>, GraphQLError>
  where
    K: for<'de> Deserialize<'de>,
  {
    self.query_with_vars::<K, ()>(query, ()).await
  }

  pub async fn query_unwrap<K>(&self, query: &str) -> Result<K, GraphQLError>
  where
    K: for<'de> Deserialize<'de>,
  {
    self.query_with_vars_unwrap::<K, ()>(query, ()).await
  }

  pub async fn query_with_vars_unwrap<K, T: Serialize>(
    &self,
    query: &str,
    variables: T,
  ) -> Result<K, GraphQLError>
  where
    K: for<'de> Deserialize<'de>,
  {
    match self.query_with_vars(query, variables).await? {
      Some(v) => Ok(v),
      None => Err(GraphQLError::with_text(
        "No data from graphql server for this query",
      )),
    }
  }

  #[cfg(target_arch = "wasm32")]
  fn client(&self) -> Result<reqwest::Client, GraphQLError> {
    Ok(Client::new())
  }

  #[cfg(not(target_arch = "wasm32"))]
  fn client(&self) -> Result<reqwest::Client, GraphQLError> {
    Ok(
      Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(|e| GraphQLError::with_text(format!("Can not create client: {:?}", e)))?,
    )
  }

  pub async fn query_with_vars<K, T: Serialize>(
    &self,
    query: &str,
    variables: T,
  ) -> Result<Option<K>, GraphQLError>
  where
    K: for<'de> Deserialize<'de>,
  {
    let client: reqwest::Client = self.client()?;
    let body = RequestBody {
      query: query.to_string(),
      variables,
    };

    let request = client
      .post(&self.endpoint)
      .json(&body)
      .headers(self.header_map.clone());

    let raw_response = request.send().await?;
    let response_body_text = raw_response
      .text()
      .await
      .map_err(|e| GraphQLError::with_text(format!("Can not get response: {:?}", e)))?;

    let json: GraphQLResponse<K> = serde_json::from_str(&response_body_text).map_err(|e| {
      GraphQLError::with_text(format!(
        "Failed to parse response: {:?}. The response body is: {}",
        e, response_body_text
      ))
    })?;

    // Check if error messages have been received
    if json.errors.is_some() {
      return Err(GraphQLError::with_json(json.errors.unwrap_or_default()));
    }

    Ok(json.data)
  }
}
