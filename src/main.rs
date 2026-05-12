use axum::{
    Router,
    http::{HeaderValue, Method}
};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use seboso_back::{
    routes::make_routes, utils::load_env_vars,
};
use tokio_postgres::NoTls;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_env_vars()?;

    let conn_manager = PostgresConnectionManager::new_from_stringlike(
        &format!(
            "host=db user={} dbname={} password={}",
            config.dbuser, config.dbname, config.dbpwd
        ),
        NoTls,
    )?;

    let conn_pool = Pool::builder().build(conn_manager).await?;

    let cors = CorsLayer::new()
    .allow_origin(config.cors_allowed.parse::<HeaderValue>()?)
    .allow_methods([
        Method::GET,
        Method::POST,
        Method::PUT,
        Method::DELETE,
    ])
    .allow_headers(Any);

    let app = Router::new()
        .merge(make_routes())
        .with_state(conn_pool)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    println!("Listening on http://localhost:3000");

    Ok(())
}
