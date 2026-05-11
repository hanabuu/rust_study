use std::error::Error;

/// 同期的にURLをフェッチする関数
/// 
/// # Errors
/// - ネットワークエラーやJSONのパースエラーが発生する可能性があります。
/// # Examples
/// ```
/// sync_fetch_url(&fetch_url).unwrap();
/// ```
pub fn sync_fetch_url(target: &str) -> Result<(), Box<dyn Error>> {
  let response = reqwest::blocking::get(target)?;
  let status = response.status();
  let json: serde_json::Value = response.json()?;

  println!("URL: {}", target);
  println!("HTTP Status: {}", status);
  println!("Response JSON: {}", json);

  Ok(())
}

/// 非同期的にURLをフェッチする関数
/// 
/// # Errors
/// - ネットワークエラーやJSONのパースエラーが発生する可能性があります。
/// # Examples
/// ```
/// let runtime = Runtime::new().unwrap();
/// runtime.block_on(async_fetch_url()).unwrap();
/// ```
pub async fn async_fetch_url(target: &str) -> Result<(), Box<dyn Error>> {
  let client = reqwest::Client::new();  
  let response = client.get(target).send().await?;
  let status = response.status();
  let json: serde_json::Value = response.json().await?;

  println!("URL: {}", target);
  println!("HTTP Status: {}", status);
  println!("Response JSON: {}", json);

  Ok(())
}