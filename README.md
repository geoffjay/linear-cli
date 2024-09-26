# Linear CLI

> [!WARNING]
> This is a WIP, might 🔥 your 🏠 down, don't use it

A simple CLI for managing issues and pull requests in Linear using GraphQL.

## Prerequisites

- Linear API Token (https://linear.app/settings/api)

## Development

### Updating the GraphQL Schema

> [!NOTE]
> The generate might not be necessary if the schema is handled by the client in queries.rs

```shell
cargo install graphql_client_cli
graphql-client introspect-schema https://api.linear.app/graphql --output schema.json
graphql-client generate \
  --schema-path=schema.json \
  --custom-scalars-module='crate::graphql::custom_scalars' \
  --response-derives='Debug' \
  --output-directory src/ \
  src/*.graphql
```

### Environment Variables

```shell
cp .env.example .env
```

## Usage

This just does one thing for now, list issues.

```shell
cargo run -- issues list
```