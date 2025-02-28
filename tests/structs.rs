use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct NodeList<T> {
  pub data: Vec<T>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Post {
  pub id: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct SinglePost {
  pub post: Post,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct AllPosts {
  pub posts: NodeList<Post>,
}

pub mod inputs {
  use serde::Serialize;

  #[derive(Serialize, Debug)]
  pub struct SinglePostVariables {
    pub id: u32,
  }
}
