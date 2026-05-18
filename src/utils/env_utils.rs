use dotenv::{dotenv, var};

pub struct Config {
    pub dbuser: String,
    pub dbname: String,
    pub dbpwd: String,
    pub secret_key: String,
    pub cors_allowed: String
}

pub fn load_env_vars() -> Result<Config, Box<dyn std::error::Error>> {
    dotenv()?;

    Ok(Config {
        dbname: var("POSTGRES_DB")?,
        dbuser: var("POSTGRES_USER")?,
        dbpwd: var("POSTGRES_PASSWORD")?,
        secret_key: var("SECRET_KEY")?,
        cors_allowed: var("CORS_ALLOWED")?
    })
}