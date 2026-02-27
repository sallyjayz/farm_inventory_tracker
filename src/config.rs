use crate::errors::AppError;
use std::{env, fmt::Display, str::FromStr};

pub struct AppConfig {
    pub database_url: String,
    pub server_port: u16,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, AppError> {
        let database_url = get_env_var("DATABASE_URL")?;
        let server_port = get_env_var("SERVER_PORT")?;

        Ok(Self {
            database_url,
            server_port,
        })
    }
}

fn get_env_var<T: FromStr>(key: &str) -> Result<T, AppError>
where
    T::Err: Display,
{
    let value = env::var(key).map_err(|_| AppError::MissingEnvironmentVarible(key.to_string()))?;
    value.trim().parse::<T>().map_err(|e| {
        AppError::ParsingError(format!(
            "Failed to parse environment variable '{}': {}",
            key, e
        ))
    })
}