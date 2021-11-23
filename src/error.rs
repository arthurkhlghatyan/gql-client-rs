use reqwest::Error;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::{self, Formatter};

pub struct GraphQLError {
  message: String,
  json: Option<Vec<GraphQLErrorMessage>>,
}

// https://spec.graphql.org/June2018/#sec-Errors
#[derive(Deserialize, Debug)]
pub struct GraphQLErrorMessage {
  message: String,
  locations: Option<Vec<GraphQLErrorLocation>>,
  extensions: Option<HashMap<String, String>>,
  path: Option<Vec<GraphQLErrorPathParam>>,
}

#[derive(Deserialize, Debug)]
pub struct GraphQLErrorLocation {
  line: u32,
  column: u32,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum GraphQLErrorPathParam {
  String(String),
  Number(u32),
}

impl GraphQLError {
  pub fn from_str(message: impl AsRef<str>) -> Self {
    Self {
      message: message.as_ref().to_string(),
      json: None,
    }
  }

  pub fn from_json(json: Vec<GraphQLErrorMessage>) -> Self {
    Self {
      message: String::from("Look at json field for more details"),
      json: Some(json),
    }
  }

  pub fn message(&self) -> &str {
    &self.message
  }

  pub fn json(&self) -> &Option<Vec<GraphQLErrorMessage>> {
    &self.json
  }
}

fn format(err: &GraphQLError, f: &mut Formatter<'_>) -> fmt::Result {
  // Print the main error message
  writeln!(f, "\nGQLClient Error: {}", err.message)?;

  // Check if query errors have been received
  if err.json.is_none() {
    return Ok(());
  }

  let errors = err.json.as_ref();

  for err in errors.unwrap() {
    writeln!(f, "Message: {}", err.message)?;
  }

  Ok(())
}

impl fmt::Display for GraphQLError {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    format(self, f)
  }
}

impl fmt::Debug for GraphQLError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    format(&self, f)
  }
}

impl std::convert::From<reqwest::Error> for GraphQLError {
  fn from(error: Error) -> Self {
    Self {
      message: error.to_string(),
      json: None,
    }
  }
}
