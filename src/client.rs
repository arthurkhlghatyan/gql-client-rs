use std::collections::HashMap;

use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::error::{GraphQLError, GraphQLErrorMessage};
use crate::ClientConfig;

#[derive(Clone, Debug)]
pub struct GQLClient {
  config: ClientConfig,
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
  fn client(&self) -> Result<Client, GraphQLError> {
    Ok(Client::new())
  }

  #[cfg(not(target_arch = "wasm32"))]
  fn client(&self) -> Result<Client, GraphQLError> {
    Client::builder()
      .timeout(std::time::Duration::from_secs(
        self.config.timeout.unwrap_or(5),
      ))
      .build()
      .map_err(|e| GraphQLError::with_text(format!("Can not create client: {:?}", e)))
  }
}

impl GQLClient {
  pub fn new(endpoint: impl AsRef<str>) -> Self {
    Self {
      config: ClientConfig {
        endpoint: endpoint.as_ref().to_string(),
        timeout: None,
        headers: Default::default(),
      },
    }
  }

  pub fn new_with_headers(
    endpoint: impl AsRef<str>,
    headers: HashMap<impl ToString, impl ToString>,
  ) -> Self {
    let _headers: HashMap<String, String> = headers
      .iter()
      .map(|(name, value)| (name.to_string(), value.to_string()))
      .into_iter()
      .collect();
    Self {
      config: ClientConfig {
        endpoint: endpoint.as_ref().to_string(),
        timeout: None,
        headers: Some(_headers),
      },
    }
  }

  pub fn new_with_config(config: ClientConfig) -> Self {
    Self { config }
  }
}

impl GQLClient {
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
        self.config.endpoint
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
    let client: Client = self.client()?;
    let body = RequestBody {
      query: query.to_string(),
      variables,
    };

    let mut request = client.post(&self.config.endpoint).json(&body);
    if let Some(headers) = &self.config.headers {
      if !headers.is_empty() {
        for (name, value) in headers {
          request = request.header(name, value);
        }
      }
    }

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
