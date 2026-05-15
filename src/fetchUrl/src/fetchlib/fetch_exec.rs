use std::error::Error;
use reqwest::{
  header::{HeaderMap, HeaderName, HeaderValue},
    Client,
};

/// ヘッダー付与なしのHTTPクライアントを生成する
/// 
/// # Errors
/// - クライアントの生成に失敗する可能性があります。 
/// # Examples
/// ```
/// let client = create_client_with_headers(&[]).unwrap();
/// ```
pub fn create_client() -> Result<Client, String> {
  let client = Client::builder()
    .build()
    .map_err(|error| format!("Failed to create HTTP client: {}", error))?;
  Ok(client)
}

/// ヘッダーを付与したHTTPクライアントを生成する
/// 
/// # 引数
/// - `custom_headers`: ヘッダー情報の配列 (`Vec<(String, String)>` など)
/// 
/// # Errors
/// - ヘッダーの生成に失敗する可能性があります。
/// 
/// # Examples
/// ```
/// let custom_headers = vec![
///   ("User-Agent".to_string(), "My Rust Client".to_string()),
///   ("Accept".to_string(), "application/json".to_string()),
/// ];
/// let client = create_client_with_headers(&custom_headers).unwrap();
/// ```
pub fn create_client_with_headers(custom_headers: &[(String, String)]) -> Result<Client, String> {
  let mut headers: HeaderMap = HeaderMap::new();
  for (key, value) in custom_headers {
    let key: HeaderName = key
      .parse()
      .map_err(|error| format!("Invalid header key '{}': {}", key, error))?;
    let value = HeaderValue::from_str(value)
      .map_err(|error| format!("Invalid header value for key '{}': {}", key.as_str(), error))?;
    headers.insert(key, value);
  }

  let client = Client::builder()
    .default_headers(headers)
    .build()
    .map_err(|error| format!("Failed to create HTTP client: {}", error))?;

  Ok(client)
}

/// 同期的にURLをフェッチする関数
/// 
/// # 引数
/// - `target`: フェッチするURLの文字列
/// 
/// # Errors
/// - ネットワークエラーやJSONのパースエラーが発生する可能性があります。
/// # Examples
/// ```
/// sync_fetch_url(&fetch_url).unwrap();
/// ```
pub fn sync_fetch_url(target: &str) -> Result<serde_json::Value, Box<dyn Error>> {
  let response = reqwest::blocking::get(target)?;
  let status = response.status();

  if !status.is_success() {
    return Err(format!("Failed to fetch URL: {}, HTTP Status: {}", target, status).into());
  }

  let json: serde_json::Value = response.json()?;

  // println!("URL: {}", target);
  // println!("HTTP Status: {}", status);
  // println!("Response JSON: {}", json);

  Ok(json)
}

/// ヘッダー等を設定済みのクライアントで非同期フェッチする関数
///
/// # 引数
/// - `client`: ヘッダー等を設定済みのHTTPクライアント
/// - `target`: フェッチするURLの文字列
/// - `query`: クエリパラメータの配列 (`Vec<(&str, u32)>` など)
/// 
/// # Errors
/// - ネットワークエラーやJSONのパースエラーが発生する可能性があります。
///
/// # Examples
/// ```
/// let runtime = Runtime::new().unwrap();
/// runtime.block_on(async_fetch_url_with_client(&client, &fetch_url, &[])).unwrap();
/// ```
pub async fn async_fetch_url_with_client(
  client: &Client,
  target: &str,
  query: &[(&str, u32)]
) -> Result<serde_json::Value, Box<dyn Error>> {
  let response = client.get(target).query(query).send().await?;
  let status = response.status();

  if !status.is_success() {
    return Err(format!("Failed to fetch URL: {}, HTTP Status: {}", target, status).into());
  }

  let json: serde_json::Value = response.json().await?;

  // println!("URL: {}", target);
  // println!("HTTP Status: {}", status);
  // println!("Response JSON: {}", json);

  Ok(json)
}