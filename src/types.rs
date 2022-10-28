use std::collections::HashMap;
#[cfg(not(target_arch = "wasm32"))]
use std::convert::TryFrom;

use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
use crate::GraphQLError;

/// GQL client config
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClientConfig {
  /// the endpoint about graphql server
  pub endpoint: String,
  /// gql query timeout, unit: seconds
  pub timeout: Option<u64>,
  /// additional request header
  pub headers: Option<HashMap<String, String>>,
  /// request proxy
  pub proxy: Option<GQLProxy>,
}

/// proxy type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ProxyType {
  Http,
  Https,
  All,
}

/// proxy auth, basic_auth
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProxyAuth {
  pub username: String,
  pub password: String,
}

/// request proxy
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GQLProxy {
  /// schema, proxy url
  pub schema: String,
  /// proxy type
  pub type_: ProxyType,
  /// auth
  pub auth: Option<ProxyAuth>,
}

#[cfg(not(target_arch = "wasm32"))]
impl TryFrom<GQLProxy> for reqwest::Proxy {
  type Error = GraphQLError;

  fn try_from(gql_proxy: GQLProxy) -> Result<Self, Self::Error> {
    let proxy = match gql_proxy.type_ {
      ProxyType::Http => reqwest::Proxy::http(gql_proxy.schema),
      ProxyType::Https => reqwest::Proxy::https(gql_proxy.schema),
      ProxyType::All => reqwest::Proxy::all(gql_proxy.schema),
    }
    .map_err(|e| Self::Error::with_text(format!("{:?}", e)))?;
    Ok(proxy)
  }
}
