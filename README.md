# Pinocchio Solana program template

This is a template for optimized Solana program development using the Pinocchio framework. This template was built by [Exo Technologies](https://exotechnologies.xyz).

## Build

From project root

```
cargo build-sbf
```

## Generating IDL

This repository uses Shank for IDL generation.

Install the Shank CLI

```
cargo install shank-cli
```

Generate IDL

```
shank idl -r program -o idl
// OR
yarn generate-idl
```

## Generating Clients

_Ensure the IDL has been created or updated using the above IDL generation steps._

Install dependencies

```
yarn install
```

Run client generation script

```
yarn generate-clients
```

## Running Tests

Integration tests are written using [LiteSvm](https://github.com/LiteSVM/litesvm). To run integration tests, from project root build and then run

```
cargo test
```
