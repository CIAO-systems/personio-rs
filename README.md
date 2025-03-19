[![Workflow Status](https://github.com/CIAO-systems/personio-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/CIAO-systems/personio-rs/actions/workflows/rust.yml)
[![GitHub License](https://img.shields.io/github/license/CIAO-systems/personio-rs?style=flat)](https://github.com/CIAO-systems/personio-rs?tab=Apache-2.0-1-ov-file)


# Personio API

## Usage 
To use the library in your project, add it to the dependencies:
```toml
[dependencies]
personio-rs = { git = "https://github.com/CIAO-systems/personio-rs" }
```
## Authentication form environment
The function `personio_rs::auth::login_from_env` takes the client credentials from the environment variables:
| Variable | Description |
| --- | --- |
| PERSONIO_CLIENT_ID | The client id from Personio |
| PERSONIO_CLIENT_SECRET | The client secret from Persnio |

## Example
Here is an example, of how to use the library to authenticate with the credentials from the environment and then list all the names of the comany employees:
```rust
use dotenv::dotenv;
use personio_rs::{
    auth::login_from_env,
    personnel::apis::{configuration::Configuration, employees_api::company_employees_get},
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    match login_from_env().await {
        Ok(token) => list_employees(token).await,
        Err(e) => eprintln!("Error logging in: {}", e),
    }
}

async fn list_employees(token: String) {
    let mut configuration = Configuration::new();
    configuration.bearer_access_token = Some(token);

    let x_personio_partner_id = "CIAO Systems GmbH";
    let x_personio_app_id = "Example Personio list employees program";
    let limit = 10;

    match company_employees_get(
        &configuration,
        Some(x_personio_partner_id),
        Some(x_personio_app_id),
        Some(limit),
        None,
        None,
        None,
        None,
    )
    .await
    {
        Ok(employees) => print_employees(employees),
        Err(e) => println!("Error: {}", e),
    }
}

fn print_employees(employees: personio_rs::personnel::models::EmployeesResponse) {
    if employees.success {
        for data in employees.data {
            if let Some(employee) = data.attributes {
                let id = employee
                    .id
                    .unwrap_or_default()
                    .value
                    .unwrap_or_default()
                    .unwrap_or_default();
                let preferred_name = employee
                    .preferred_name
                    .unwrap_or_default()
                    .value
                    .unwrap_or_default()
                    .unwrap_or_default();
                println!("{}: {}", id, preferred_name.as_str().unwrap_or("<no name>"));
            }
        }
    }
}
```

## Generating the rust stubs
To generate Rust code from the Personio-API, the [OpenAPI Generator](https://openapi-generator.tech/) is used. The documentation for the Rust configuration can be found [here](https://openapi-generator.tech/docs/generators/rust)
Here is the official [Personio API documentation](https://developer.personio.de/docs/getting-started-with-the-personio-api)

```bash
# Call the script openapi-generator (using the docker image of OpenAPI-Generator)
$> ./openapi-generator 
```

### OpenAPI specs
All API docs for Personio can be found [here](https://github.com/personio/api-docs)
* Personio-Auth API: https://github.com/personio/api-docs/blob/master/personio-auth-api.yaml
* Personio-Personnel API: https://github.com/personio/api-docs/blob/master/personio-personnel-data-api-oa3.yaml

### Options
To combine the two libraries in one, each has to have its unique package name. This can be achieved by providing the additional property `packageName`
| Library | `packageName` |
|---|---|
| personio-auth | personio-auth |
| personio-personnel | personio-personnel |
