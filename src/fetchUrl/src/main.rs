use std::error::Error;

const TARGET_URL: &str = "http://10.116.164.94:3001/api/getDbDirFile";

fn main() -> Result<(), Box<dyn Error>> {
	let response = reqwest::blocking::get(TARGET_URL)?;
	let status = response.status();
	let json: serde_json::Value = response.json()?;

	println!("URL: {}", TARGET_URL);
	println!("HTTP Status: {}", status);
	println!("Response JSON: {}", json);

	Ok(())
}
