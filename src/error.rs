use reqwest::Error;
use serde::export::Formatter;
use std::fmt;

pub struct GraphQLError {
  message: String,
}

impl GraphQLError {
  pub fn message(&self) -> &str {
    &self.message
  }
}

impl fmt::Display for GraphQLError {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl fmt::Debug for GraphQLError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "GraphQL Client Error: {{ message: {}, file: {}, line: {} }}",
      self.message,
      file!(),
      line!()
    )
  }
}

impl std::convert::From<reqwest::Error> for GraphQLError {
  fn from(error: Error) -> Self {
    Self {
      message: error.to_string(),
    }
  }
}
