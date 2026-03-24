use tokio_postgres::{NoTls, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Connection string format: host=localhost user=username password=password dbname=database
    let (client, connection) = tokio_postgres::connect("host=localhost port=5432 user=postgres dbname=sebo_project password=1234", NoTls).await?;

    // Spawn the connection to run independently
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    // Now you can execute queries using the client
    let rows = client.query("SELECT * FROM users", &[]).await?;
    println!("Connected and queried: {:?}", rows);

    Ok(())
}