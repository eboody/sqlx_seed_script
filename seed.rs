use dotenvy::dotenv;
use sqlx::postgres::{PgPool, PgQueryResult};

#[derive(Debug)]
pub struct User {
    id: i32,
    first_name: String,
    last_name: String,
    email: String,
    address_id: i32,
}

#[derive(Debug)]
pub struct Address {
    id: i32,
    street: String,
    house_number: String,
    city_id: i32,
}

#[derive(Debug)]
pub struct City {
    id: i32,
    name: String,
}

const NUMBER_OF_ROWS: u8 = 10;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let connection_string =
        std::env::var("DATABASE_URL").expect("database connection string not present as env var");

    let pool = PgPool::connect(&connection_string).await.unwrap();

    let ids: Vec<i32> = (1..=NUMBER_OF_ROWS).collect();

    for id in ids {
        create_cities(&pool, id).await?;
        create_addresses(&pool, id).await?;
        create_users(&pool, id).await?;
    }

    Ok(())
}

async fn create_cities(pool: &PgPool, id: i32) -> Result<PgQueryResult, sqlx::Error> {
    let city = City {
        id,
        name: format!("City{id}"),
    };

    sqlx::query!(
        r#"
            INSERT INTO cities (id, name) VALUES ($1, $2);
        "#,
        city.id,
        city.name,
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
        address.id,
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
        user.id,
        user.first_name,
        user.last_name,
        user.email,
        user.address_id
    )
    .execute(pool)
    .await
}
