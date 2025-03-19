use std::{env, process::exit};

use dotenv::dotenv;
use personio_auth::{
    apis::{auth_api::auth_post, configuration::Configuration},
    models::AuthPostRequest,
};

static PERSONIO_CLIENT_ID: &str = "PERSONIO_CLIENT_ID";
static PERSONIO_CLIENT_SECRET: &str = "PERSONIO_CLIENT_SECRET";

#[tokio::main]
async fn main() {
    dotenv().ok();

    if let Ok(client_id) = env::var(PERSONIO_CLIENT_ID) {
        if let Ok(client_secret) = env::var(PERSONIO_CLIENT_SECRET) {
            let configuration = Configuration::new();
            let auth_post_request = AuthPostRequest::new(client_id, client_secret);

            match auth_post(&configuration, Some(auth_post_request)).await {
                Ok(auth_response) => {
                    if auth_response.success {
                        println!("{}", auth_response.data.token);
                    }
                }
                Err(e) => {
                    eprintln!("Error while calling auth_post: {e}");
                    exit(2);
                }
            }
        } else {
            env_not_present(PERSONIO_CLIENT_SECRET);
        }
    } else {
        env_not_present(PERSONIO_CLIENT_ID);
    }
}

fn env_not_present(variable: &str) {
    eprintln!("Environment variable '{}' missing", variable);
    exit(1);
}
