mod fetchlib;

use std::error::Error;
use tokio::runtime::Runtime;

use dotenvy::dotenv;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {

  dotenv().expect(".env file not found");
  let fetch_url = env::var("FetchURL").expect("FetchURL not set");

  println!("=== Sync request ===");
  let json: serde_json::Value = fetchlib::fetch_exec::sync_fetch_url(&fetch_url)?;

  println!("json: {}", json);

  println!("=== Async request ===");
  let client = fetchlib::fetch_exec::create_client()
    .map_err(|e| std::io::Error::other(format!("failed to create client: {}", e)))?;

  let runtime = Runtime::new()?;
  let json: serde_json::Value = runtime.block_on(
    fetchlib::fetch_exec::async_fetch_url_with_client(&client, &fetch_url, &[]),
  )?;

  println!("json: {}", json);

  println!("=== get gitlab api ===");
  let custom_headers = vec![
    ("PRIVATE-TOKEN".to_string(), "XXXXXXXXX".to_string()),
  ];
  let client = fetchlib::fetch_exec::create_client_with_headers(&custom_headers)
    .map_err(|e| std::io::Error::other(format!("failed to create client: {}", e)))?;

  let gitlab_api_url = "http://localhost/api/v4/users/24/contributed_projects";
  let mut query_pairs: Vec<(&str, u32)> = Vec::new();
  query_pairs.push(("page", 1));
  query_pairs.push(("per_page", 10));

  let json: serde_json::Value = runtime.block_on(
    fetchlib::fetch_exec::async_fetch_url_with_client(&client, gitlab_api_url, &query_pairs),
  )?;

  let requested_url = fetchlib::fetch_param::build_requested_url(gitlab_api_url, &query_pairs);

  println!("Requested URL: {}", requested_url);
  println!("json: {}", json);

  Ok(())
}
