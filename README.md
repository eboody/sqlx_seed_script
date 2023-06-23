# SQLx Seed Script
This is a simple Rust script to seed a PostgreSQL database using SQLx.

## Overview
This script creates a series of cities, addresses, and users in the database.  Each entity has an associated 'id', and the script creates entities with ids from 1 to 5.

**Note**: The included `User`, `Address`, and `City` structs in this script are mock structures that correspond to a hypothetical migration file that created 'cities', 'addresses', and 'users' tables. Adjust these structures according to your own migration files and database design.

## Prerequisites
Ensure you have the following installed:

- sqlx
- tokio
- dotenvy

You'll probably already have these installed for your crate but in case you don't you can install these like this:

```sh
cargo add dotenvy
cargo add sqlx --features "postgres,runtime-tokio-native-tls"
cargo add tokio --features "macros,rt-multi-thread"
```

## Usage
Ensure you have a `.env` file in your project root with a `DATABASE_URL` variable. This should be a connection string for your PostgreSQL database.

Navigate to the `/src/bin` directory and run the script using cargo:

```sh
cargo run --bin seed
```

This will run the main function, which creates a connection pool with the database and uses it to create cities, addresses, and users. Again these are mock entities. Replace these with your own.

## Troubleshooting
If you encounter errors, ensure that:

- Your `DATABASE_URL` connection string is correct.
- Your PostgreSQL server is running and accepting connections.
- The relevant tables exist in your database. 'cities', 'addresses', and 'users' in this example.
