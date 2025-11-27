use dotenvy::dotenv;
use std::env;

fn main() {
    // load environment variables from .env file
    dotenv().expect(".env file not found");
    for (key, value) in env::vars() {
        if key.starts_with("APP_") {
            println!("{key}: {value}");
        }
    }
}