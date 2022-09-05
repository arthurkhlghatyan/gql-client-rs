mod structs;

use crate::structs::{inputs::SinglePostVariables, AllPosts, SinglePost};
use gql_client::Client;
use std::collections::HashMap;

// Initialize endpoint
const ENDPOINT: &'static str = "https://graphqlzero.almansi.me/api";

#[tokio::test]
pub async fn fetches_one_post() {
  let client = Client::new(ENDPOINT, 5);

  let query = r#"
    query SinglePostQuery($id: ID!) {
      post(id: $id) {
        id
      }
    }
  "#;

  let variables = SinglePostVariables { id: 2 };
  let data = client
    .query_with_vars_unwrap::<SinglePost, SinglePostVariables>(query, variables)
    .await
    .unwrap();

  assert_eq!(data.post.id, String::from("2"), "Post id retrieved 2");
}

#[tokio::test]
pub async fn fetches_all_posts() {
  let mut headers = HashMap::new();
  headers.insert("content-type", "application/json");

  let client = Client::new_with_headers(ENDPOINT, 5, headers);

  let query = r#"
    query AllPostsQuery {
      posts {
        data {
          id
        }
      }
    }
  "#;

  let data: AllPosts = client.query_unwrap::<AllPosts>(query).await.unwrap();

  assert!(data.posts.data.len() > 0 as usize);
}
