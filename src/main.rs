use dotenv::{dotenv, var};
use tokio_postgres::{NoTls};
use axum::{
    Router
};

mod routes;

struct Config {
    dbuser: String,
    dbname: String,
    dbpwd: String,
}

fn load_env_vars(
) -> Result<Config, Box<dyn std::error::Error>> {
    dotenv()?;

    Ok(Config{
        dbname: var("POSTGRES_DB")?,
        dbuser: var("POSTGRES_USER")?,
        dbpwd: var("POSTGRES_PASSWORD")?,
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_env_vars()?;

    let (client, connection) = tokio_postgres::connect(
        &format!(
            "host=localhost user={} dbname={} password={}",
            config.dbuser, config.dbname, config.dbpwd
        ),
        NoTls,
    )
    .await?;

    // Spawn the connection to run independently
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    client
        .query(
            "INSERT INTO users (name, email, login, password, cell_number, role, created_at, updated_at)
            VALUES
            ('João Silva', 'joao.silva@email.com', 'joaosilva', 'senha123', '85999990001', 1, NOW(), NOW()),
            ('Maria Oliveira', 'maria.oliveira@email.com', 'mariaoliveira', 'senha123', '85999990002', 2, NOW(), NOW()),
            ('Carlos Souza', 'carlos.souza@email.com', 'carlossouza', 'senha123', '85999990003', 1, NOW(), NOW()),
            ('Ana Santos', 'ana.santos@email.com', 'anasantos', 'senha123', '85999990004', 3, NOW(), NOW()),
            ('Pedro Lima', 'pedro.lima@email.com', 'pedrolima', 'senha123', '85999990005', 2, NOW(), NOW());",
            &[],
        )
        .await?;

    let app = Router::new()
        .merge(routes::make_routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    
    println!("Listening on http://localhost:3000");

    Ok(())
}
