pub use personio_auth::*;

use std::{env, error::Error};

use personio_auth::{
    apis::{auth_api::auth_post, configuration::Configuration},
    models::AuthPostRequest,
};

static PERSONIO_CLIENT_ID: &str = "PERSONIO_CLIENT_ID";
static PERSONIO_CLIENT_SECRET: &str = "PERSONIO_CLIENT_SECRET";

/// Login with credentials taken from the environment variables.
///
/// * The `client_id` is taken from the environment variable `PERSONIO_CLIENT_ID`
/// * The `client_secret` is taken from the environment variable `PERSONIO_CLIENT_SECRET`
pub async fn login_from_env() -> Result<String, Box<dyn Error>> {
    match EnvironmentCredentials::new() {
        Ok(env) => login(env.client_id, env.client_secret).await,
        Err(e) => Err(e),
    }
}

/// Login with client id and client secret. On succes the function returns the `token`
///
/// # Arguments
/// * `client_id`: The Personio client id
/// * `client_secret`: The Personio client secret
///
pub async fn login(client_id: String, client_secret: String) -> Result<String, Box<dyn Error>> {
    let configuration = Configuration::new();
    let auth_post_request = AuthPostRequest::new(client_id, client_secret);

    match auth_post(&configuration, Some(auth_post_request)).await {
        Ok(auth_response) => {
            if auth_response.success {
                Ok(auth_response.data.token)
            } else {
                Err("Authentication failed".into())
            }
        }
        Err(e) => Err(e.into()),
    }
}

pub struct EnvironmentCredentials {
    pub client_id: String,
    pub client_secret: String,
}

impl EnvironmentCredentials {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            client_id: env::var(PERSONIO_CLIENT_ID)
                .map_err(|e| format!("{PERSONIO_CLIENT_ID}: {e}"))?,
            client_secret: env::var(PERSONIO_CLIENT_SECRET)
                .map_err(|e| format!("{PERSONIO_CLIENT_SECRET}: {e}"))?,
        })
    }
}
