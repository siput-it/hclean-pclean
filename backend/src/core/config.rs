use std::{env, io, net::SocketAddr};

pub struct Config {
    pub bind_addr: SocketAddr,
    pub database_url: Option<String>,
    pub database_max_connections: u32,
    pub database_connect_timeout_seconds: u64,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        match dotenvy::dotenv() {
            Ok(_) => {}
            Err(dotenvy::Error::Io(error)) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(Box::new(error)),
        }

        let bind_addr = env::var("BIND_ADDR")
            .unwrap_or_else(|_| "127.0.0.1:3000".to_owned())
            .parse::<SocketAddr>()
            .map_err(|error| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("BIND_ADDR harus berupa alamat socket yang valid: {error}"),
                )
            })?;

        let database_url = env::var("DATABASE_URL")
            .ok()
            .filter(|value| !value.trim().is_empty());

        let database_max_connections = env::var("DATABASE_MAX_CONNECTIONS")
            .unwrap_or_else(|_| "5".to_string())
            .parse::<u32>()
            .map_err(|error| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("DATABASE_MAX_CONNECTIONS harus berupa angka bulat: {error}"),
                )
            })?;

        let database_connect_timeout_seconds = env::var("DATABASE_CONNECT_TIMEOUT_SECONDS")
            .unwrap_or_else(|_| "5".to_string())
            .parse::<u64>()
            .map_err(|error| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("DATABASE_CONNECT_TIMEOUT_SECONDS harus berupa angka bulat: {error}"),
                )
            })?;

        Ok(Self {
            bind_addr,
            database_url,
            database_max_connections,
            database_connect_timeout_seconds,
        })
    }
}
