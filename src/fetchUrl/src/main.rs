mod fetchlib;

use std::error::Error;
use tokio::runtime::Runtime;

use dotenvy::dotenv;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {

  dotenv().expect(".env file not found");
  let fetch_url = env::var("FetchURL").expect("FetchURL not set");

  println!("=== Sync request ===");
  fetchlib::fetch_lib::sync_fetch_url(&fetch_url)?;

  println!("=== Async request ===");
  let runtime = Runtime::new()?;
  runtime.block_on(fetchlib::fetch_lib::async_fetch_url(&fetch_url))?;

  Ok(())
}
