use reqwest::Url;
use std::str::FromStr;

#[test]
fn test_url() {
  let url_raw = "https://subql.darwinia.network/subql-bridger-darwinia";
  let url = Url::from_str(url_raw).unwrap();
  let schema = url.scheme();
  let host = url.host().unwrap();
  assert_eq!(
    "https://subql.darwinia.network",
    format!("{}://{}", schema, host)
  );
}
