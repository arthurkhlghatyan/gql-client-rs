use reqwest::Error;
use serde::{ Deserialize, export::Formatter };
use std::fmt;
use std::collections::HashMap;

pub struct GraphQLError {
  message: String,
  json: Option<Vec<GraphQLErrorMessage>>
}

// https://spec.graphql.org/June2018/#sec-Errors
#[derive(Deserialize, Debug)]
pub struct GraphQLErrorMessage {
  message: String,
  locations: Vec<GraphQLErrorLocation>,
  extensions: HashMap<String, String>
}

#[derive(Deserialize, Debug)]
pub struct GraphQLErrorLocation {
  line: u32,
  column: u32
}

impl GraphQLError {
  pub fn from_str(message: &str) -> Self {
    Self {
      message: String::from(message),
      json: None
    }
  }

  pub fn from_json(json: Vec<GraphQLErrorMessage>) -> Self {
    Self {
      message: String::from(""),
      json: Some(json)
    }
  }

  pub fn message(&self) -> &str {
    &self.message
  }

  pub fn json(self) -> Option<Vec<GraphQLErrorMessage>> {
    self.json
  }
}

impl fmt::Display for GraphQLError {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "GQLClient Error {}", self.message)
  }
}

impl fmt::Debug for GraphQLError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "GQLClient Error: {}", self.message)
  }
}

impl std::convert::From<reqwest::Error> for GraphQLError {
  fn from(error: Error) -> Self {
    Self {
      message: error.to_string(),
      json: None
    }
  }
}
