use reqwest::Error;
use serde::{export::Formatter, Deserialize};
use std::collections::HashMap;
use std::fmt;

pub struct GraphQLError {
  message: String,
  json: Option<Vec<GraphQLErrorMessage>>,
}

// https://spec.graphql.org/June2018/#sec-Errors
#[derive(Deserialize, Debug)]
pub struct GraphQLErrorMessage {
  message: String,
  locations: Vec<GraphQLErrorLocation>,
  extensions: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
pub struct GraphQLErrorLocation {
  line: u32,
  column: u32,
}

impl GraphQLError {
  pub fn from_str(message: &str) -> Self {
    Self {
      message: String::from(message),
      json: None,
    }
  }

  pub fn from_json(json: Vec<GraphQLErrorMessage>) -> Self {
    Self {
      message: String::from("Error occurred in query"),
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
  // Print the message and exit
  if err.json.is_none() {
    return writeln!(f, "\nGQLClient Error {}", err.message);
  }

  let errors = err.json.as_ref();

  writeln!(f, "\nGQLClient Error: Look at json field for more details")?;

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
