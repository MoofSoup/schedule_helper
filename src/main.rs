
use dotenv::dotenv;
use std::env;



fn main() {
    dotenv().ok();

    let dify_api_key = env::var("DIFY_API_KEY").expect("DIFY_API_KEY not set");

    match env::var("DIFY_API_KEY") {
        Ok(api_key) => println!("Dify API Key: {}", api_key),
        Err(e) => println!("Couldn't read DIFY_API_KEY: {}", e),
    }
}

