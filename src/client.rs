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
  #[cfg(target_arch = "wasm32")]
  fn client(&self) -> Result<reqwest::Client, GraphQLError> {
    Ok(Client::new())
  }

  #[cfg(not(target_arch = "wasm32"))]
  fn client(&self) -> Result<reqwest::Client, GraphQLError> {
    Client::builder()
      .timeout(std::time::Duration::from_secs(5))
      .build()
      .map_err(|e| GraphQLError::with_text(format!("Can not create client: {:?}", e)))
  }
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
      None => Err(GraphQLError::with_text(format!(
        "No data from graphql server({}) for this query",
        self.endpoint
      ))),
    }
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
    let status = raw_response.status();
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

    if !status.is_success() {
      return Err(GraphQLError::with_message_and_json(
        format!("The response is [{}]", status.as_u16()),
        json.errors.unwrap_or_default(),
      ));
    }

    // Check if error messages have been received
    if json.errors.is_some() {
      return Err(GraphQLError::with_json(json.errors.unwrap_or_default()));
    }
    if json.data.is_none() {
      log::warn!(target: "gql-client", "The deserialized data is none, the response is: {}", response_body_text);
    }

    Ok(json.data)
  }
}
