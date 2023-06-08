use dotenvy::dotenv;
use sqlx::postgres::{PgPool, PgQueryResult};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: i32,
    first_name: String,
    last_name: String,
    email: String,
    address_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    id: i32,
    street: String,
    house_number: String,
    city_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct City {
    id: i32,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let connection_string =
        std::env::var("DATABASE_URL").expect("database connection string not present as env var");

    let pool = PgPool::connect(&connection_string).await.unwrap();

    let ids: Vec<i32> = (1..=5).collect();

    for id in ids {
        create_cities(&pool, id).await?;
        create_addresses(&pool, id).await?;
        create_users(&pool, id).await?;
    }

    Ok(())
}

async fn create_cities(pool: &PgPool, id: i32) -> Result<PgQueryResult, sqlx::Error> {
    let name = format!("City{id}");
    sqlx::query!(
        r#"
            INSERT INTO cities (id, name) VALUES ($1, $2);
        "#,
        id,
        name,
    )
    .execute(pool)
    .await
}

async fn create_addresses(pool: &PgPool, id: i32) -> Result<PgQueryResult, sqlx::Error> {
    let address = Address {
        id,
        street: format!("Street{id}"),
        house_number: format!("{id}"),
        city_id: id,
    };

    sqlx::query!(
        r#"
            INSERT INTO addresses (id, street, house_number, city_id) VALUES ($1, $2, $3, $4);
        "#,
        id,
        address.street,
        address.house_number,
        address.city_id,
    )
    .execute(pool)
    .await
}

async fn create_users(pool: &PgPool, id: i32) -> Result<PgQueryResult, sqlx::Error> {
    let user = User {
        id,
        first_name: format!("First{id}"),
        last_name: format!("Last{id}"),
        email: format!("user{id}@example.com"),
        address_id: id,
    };

    sqlx::query!(
        r#"
            INSERT INTO users (id, first_name, last_name, email, address_id) VALUES ($1, $2, $3, $4, $5);
        "#,
        id,
        user.first_name,
        user.last_name,
        user.email,
        user.address_id
    )
    .execute(pool)
    .await
}
