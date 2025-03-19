# Personio API
To generate Rust code from the Personio-API, the [OpenAPI Generator](https://openapi-generator.tech/) is used. The documentation for the Rust configuration can be found [here](https://openapi-generator.tech/docs/generators/rust)

## OpenAPI specs
All API docs for Personio can be found [here](https://github.com/personio/api-docs)
* Personio-Auth API: https://github.com/personio/api-docs/blob/master/personio-auth-api.yaml
* Personio-Personnel API: https://github.com/personio/api-docs/blob/master/personio-personnel-data-api-oa3.yaml

## Options
To combine the two libraries in one, each has to have its unique package name. This can be achieved by providing the additional property `packageName`
| Library | `packageName` |
|---|---|
| personio-auth | personio-auth |
| personio-personnel | personio-personnel |