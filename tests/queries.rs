use gql_client::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct NodeList<T> {
  data: Vec<T>,
}

#[derive(Deserialize, Debug)]
struct Post {
  id: String,
}

#[derive(Deserialize, Debug)]
struct SinglePost {
  post: Post,
}

#[derive(Deserialize, Debug)]
struct AllPosts {
  posts: NodeList<Post>,
}

#[derive(Serialize, Debug)]
struct SinglePostVariables {
  id: u32,
}

// Initialize endpoint
const ENDPOINT: &'static str = "https://graphqlzero.almansi.me/api";

#[tokio::test]
pub async fn fetches_one_post() {
  let client = Client::new(ENDPOINT);

  let query = r#"
    query SinglePostQuery($id: ID!) {
      post(id: $id) {
        id
      }
    }
  "#;

  let variables = SinglePostVariables { id: 2 };
  let data = client
    .query_with_vars::<SinglePost, SinglePostVariables>(
      query,
      variables
    )
    .await
    .unwrap();

  assert_eq!(data.post.id, String::from("2"), "Post id retrieved 2");
}

#[tokio::test]
pub async fn fetches_all_posts() {
  let client = Client::new(ENDPOINT);

  let query = r#"
    query AllPostsQuery {
      posts {
        data {
          id
        }
      }
    }
  "#;

  let data: AllPosts = client
    .query::<AllPosts>(query)
    .await
    .unwrap();

  assert!(data.posts.data.len() >= 0 as usize);
}
