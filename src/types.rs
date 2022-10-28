use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// GQL client config
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClientConfig {
  /// the endpoint about graphql server
  pub endpoint: String,
  /// gql query timeout, unit: seconds
  pub timeout: Option<u64>,
  /// additional request header
  pub headers: Option<HashMap<String, String>>,
}
