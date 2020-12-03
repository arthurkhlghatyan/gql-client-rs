#![cfg_attr(docsrs, feature(doc_cfg))]
//! Minimal GraphQL client for rust
//!
//! * Simple API, supports queries and mutations
//! * Does not require schema file for introspection
//!
//! # Basic Usage
//!
//! * Use client.query_with_vars for queries with variables
//! * There's also a wrapper client.query if there is no need to pass variables
//!
//! ```rust
//! use gql_client::Client;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Deserialize)]
//! pub struct Data {
//!    user: User
//! }
//!
//! #[derive(Deserialize)]
//! pub struct User {
//!     id: String,
//!     name: String
//! }
//!
//! #[derive(Serialize)]
//! pub struct Vars {
//!     id: u32
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let endpoint = "https://graphqlzero.almansi.me/api";
//!     let query = r#"
//!         query UserByIdQuery($id: ID!) {
//!             user(id: $id) {
//!                 id
//!                 name
//!             }
//!         }
//!     "#;
//!
//!     let client = Client::new(endpoint);
//!     let vars = Vars { id: 1 };
//!     let data = client.query_with_vars::<Data, Vars>(query, vars).await.unwrap();
//!
//!     println!("Id: {}, Name: {}", data.user.id, data.user.name);
//!
//!     Ok(())
//! }
//! ```
//!
//!
//! # Passing HTTP headers
//!
//! Client exposes new_with_headers function to pass headers
//! using simple HashMap<&str, &str>
//!
//! ```rust
//!use gql_client::Client;
//!use std::collections::HashMap;
//!
//!#[tokio::main]
//!async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!    let endpoint = "https://graphqlzero.almansi.me/api";
//!    let mut headers = HashMap::new();
//!    headers.insert("authorization", "Bearer <some_token>");
//!
//!    let client = Client::new_with_headers(endpoint, headers);
//!
//!    Ok(())
//!}
//! ```

mod client;
mod error;

pub use client::GQLClient as Client;
pub use error::GraphQLError;
