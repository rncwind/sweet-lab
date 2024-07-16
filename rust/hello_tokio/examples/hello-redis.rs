use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Open a connection to redis
    let mut client = client::connect("0.0.0.0:6379").await?;

    // Set the key hello to the value world
    client.set("hello", "world".into()).await?;

    // Get the key back from mini-redis
    let result = client.get("hello").await?;

    println!("Got result from the server. result={:?}", result);

    Ok(())
}
