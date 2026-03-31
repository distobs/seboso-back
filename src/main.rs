use dotenv::{dotenv, var};
use tokio_postgres::{Error, NoTls};
use axum::{
    Router,
};

mod routes;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Setup environment variables
    match dotenv() {
        Ok(_) => (),
        Err(e) => panic!("Erro dotenv: {e:?}")
    };

    let postgres_db: String = match var("POSTGRES_DB") {
        Ok(v) => v,
        Err(e) => panic!("Erro POSTGRES_DB: {e:?}")
    };

    let postgres_user: String = match var("POSTGRES_USER") {
        Ok(v) => v,
        Err(e) => panic!("Erro POSTGRES_USER: {e:?}")
    };

    let postgres_password: String = match var("POSTGRES_PASSWORD") {
        Ok(v) => v,
        Err(e) => panic!("Erro POSTGRES_PASSWORD: {e:?}")
    };

    // Connection string format: host=localhost user=username password=password dbname=database
    let (client, connection) = tokio_postgres::connect(
        format!(
            "host=localhost user={} dbname={} password={}",
            postgres_user, postgres_db, postgres_password
        )
        .as_str(),
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

    let rows = client.query("SELECT * FROM users;", &[]).await?;

    // Iterate over results
    for row in rows {
        let id: i32 = row.get("id");
        let name: String = row.get("name");
        let email: String = row.get("email");

        println!("{} | {} | {}", id, name, email);
    }

    let app = Router::new()
        .merge(routes::make_routes());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    println!("Listening on http://localhost:3000");

    Ok(())
}
