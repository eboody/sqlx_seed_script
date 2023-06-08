# SQLx Seed Script
This is a simple Rust script to seed a PostgreSQL database using SQLx.

## Overview
This script creates a series of cities, addresses, and users in the database.  Each entity has an associated 'id', and the script creates entities with ids from 1 to 5. These are intended to serve as mock data so, of course, replace these with entities from your crate.

## Structure
There are three primary entities in this script: User, Address, and City. Each of these entities has an associated struct in this script, as well as a function to create instances of it in the database.

## Prerequisites
Ensure you have the following installed:

- sqlx
- tokio
- dotenvy
- serde
- serde_json

You'll probably already have these installed for your crate but in case you don't you can install these like this:

```sh
cargo add sqlx --features "postgres,runtime-tokio-native-tls"
cargo add tokio --features "macros,rt-multi-thread"
cargo add dotenvy
cargo add serde --features "derive"
cargo add serde_json
```

## Usage
Ensure you have a .env file in your project root with a DATABASE_URL variable. This should be a connection string for your PostgreSQL database.

Add this repository as a binary in your Cargo.toml file under [dependencies] section.

Navigate to the /src/bin directory and run the script using cargo:

```rust
Copy code
cargo run --bin seed
```
This will run the main function, which creates a connection pool with the database and uses it to create cities, addresses, and users. Again these are mock entities. Replace these with your own.

## Troubleshooting
If you encounter errors, ensure that:

- Your `DATABASE_URL` connection string is correct.
- Your PostgreSQL server is running and accepting connections.
- The relevant tables exist in your database. 'cities', 'addresses', and 'users' in this example.
