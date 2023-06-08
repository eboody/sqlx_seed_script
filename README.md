# SQLx Seed Script
This is a simple Rust script to seed a PostgreSQL database using SQLx.

## Overview
This script creates a series of cities, addresses, and users in the database. Each entity has an associated 'id', and the script creates entities with ids from 1 to 5.

## Structure
There are three primary entities in this script: User, Address, and City. Each of these entities has an associated struct in this script, as well as a function to create instances of it in the database.

## Prerequisites
Ensure you have the following installed:

- Rust
- SQLx
- PostgreSQL
- dotenv

## Usage
Ensure you have a .env file in your project root with a DATABASE_URL variable. This should be a connection string for your PostgreSQL database.

Add this repository as a binary in your Cargo.toml file under [dependencies] section.

Navigate to the /src/bin directory and run the script using cargo:

```rust
Copy code
cargo run --bin seed
```
This will run the main function, which creates a connection pool with the database and uses it to create cities, addresses, and users.

## Troubleshooting
If you encounter errors, ensure that:

Your `DATABASE_URL` connection string is correct.
Your PostgreSQL server is running and accepting connections.
The 'cities', 'addresses', and 'users' tables exist in your database. (of course, change this as necessary)
