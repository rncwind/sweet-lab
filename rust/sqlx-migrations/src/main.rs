use std::collections::HashMap;

use log::info;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    env_logger::init();
    // Get us a postgres pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost/northwind")
        .await?;

    info!("Got Pool");

    // Run our migrations
    sqlx::migrate!("./migrations").run(&pool).await?;
    info!("Databases setup!");

    // Check our migrations ran
    let rows: Vec<Option<String>> = sqlx::query!("SELECT city FROM customers")
        .map(|row| row.city)
        .fetch_all(&pool)
        .await?;

    for row in rows {
        println!("{:?}", row)
    }

    Ok(())
}
