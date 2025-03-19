pub mod auth {
    pub use personio_auth::*;
}

pub mod personnel {
    pub use personio_personnel::*;
}

#[cfg(test)]
mod tests {
    use std::{env, error::Error};

    use dotenv::dotenv;
    use personio_auth::{
        apis::{auth_api::auth_post, configuration::Configuration},
        models::AuthPostRequest,
    };

    #[tokio::test]
    #[ignore = "for manual testing"]
    async fn test_auth() -> Result<(), Box<dyn Error>> {
        dotenv().ok();

        let client_id = env::var("PERSONIO_CLIENT_ID")?;
        let client_secret = env::var("PERSONIO_CLIENT_SECRET")?;

        let configuration = Configuration::new();
        let auth_post_request = AuthPostRequest::new(client_id, client_secret);

        let auth_response = auth_post(&configuration, Some(auth_post_request)).await?;

        assert!(auth_response.success);
        println!("Token: {:?}", auth_response.data.token);

        Ok(())
    }
}
