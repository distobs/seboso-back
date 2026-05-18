use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

pub type DbPool = Pool<PostgresConnectionManager<NoTls>>;