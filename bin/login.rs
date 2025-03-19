use std::process::exit;

use dotenv::dotenv;
use personio_rs::auth::login_from_env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    match login_from_env().await {
        Ok(token) => {
            println!("{}", token);
        }
        Err(e) => {
            eprintln!("Error while calling login: {e}");
            exit(1);
        }
    }
}
