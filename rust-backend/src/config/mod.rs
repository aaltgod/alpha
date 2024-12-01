use anyhow::anyhow;

#[derive(Debug)]
pub struct PostgresConfig {
    pub host: String,
    pub port: i32,
    pub username: String,
    pub password: String,
    pub database_name: String,
    pub database_url: String,
    pub max_connections: u32,
    pub timeout: std::time::Duration,
}

#[derive(Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
}

#[derive(Debug)]
pub struct SnifferConfig {
    pub interface: String,
    pub tcp_stream_ttl: chrono::Duration,
    pub max_stream_ttl: chrono::Duration,
}

pub fn provide_postgres_config() -> Result<PostgresConfig, anyhow::Error> {
    dotenv::dotenv().map_err(|e| anyhow!(e.to_string()))?;

    Ok(PostgresConfig {
        host: dotenv::var("POSTGRES_HOST")
            .map_err(|_| anyhow!("POSTGRES_HOST not found in .env"))?,
        port: dotenv::var("POSTGRES_PORT")
            .map_err(|_| anyhow!("POSTGRES_PORT not found in .env"))?
            .parse()
            .map_err(|e| anyhow!("invalid POSTGRES_PORT: {e}"))?,
        username: dotenv::var("POSTGRES_USERNAME")
            .map_err(|_| anyhow!("POSTGRES_USERNAME not found in .env"))?,
        password: dotenv::var("POSTGRES_PASSWORD")
            .map_err(|_| anyhow!("POSTGRES_PASSWORD not found in .env"))?,
        database_name: dotenv::var("POSTGRES_DATABASE")
            .map_err(|_| anyhow!("POSTGRES_DATABASE not found in .env"))?,
        database_url: dotenv::var("DATABASE_URL")
            .map_err(|_| anyhow!("DATABASE_URL not found in .env"))?,
        max_connections: dotenv::var("POSTGRES_MAX_CONNECTIONS")
            .map_err(|_| anyhow!("POSTGRES_MAX_CONNECTIONS not found in .env"))?
            .parse()
            .map_err(|e| anyhow!("invalid POSTGRES_MAX_CONNECTIONS: {e}"))?,
        timeout: std::time::Duration::from_secs(
            dotenv::var("POSTGRES_TIMEOUT")
                .map_err(|_| anyhow!("POSTGRES_TIMEOUT not found in .env"))?
                .parse()
                .map_err(|e| anyhow!("invalid POSTGRES_TIMEOUT: {e}"))?,
        ),
    })
}

pub fn provide_server_config() -> Result<ServerConfig, anyhow::Error> {
    dotenv::dotenv().map_err(|e| anyhow!(e.to_string()))?;

    Ok(ServerConfig {
        host: dotenv::var("SERVER_HOST").map_err(|_| anyhow!("SERVER_HOST not found in .env"))?,
        port: dotenv::var("SERVER_PORT")
            .map_err(|_| anyhow!("SERVER_PORT not found in .env"))?
            .parse()
            .map_err(|e| anyhow!("invalid SERVER_PORT: {e}"))?,
    })
}

pub fn provide_sniffer_config() -> Result<SnifferConfig, anyhow::Error> {
    dotenv::dotenv().map_err(|e| anyhow!(e.to_string()))?;

    Ok(SnifferConfig {
        interface: dotenv::var("SNIFFER_INTERFACE")
            .map_err(|_| anyhow!("SNIFFER_INTERFACE not found in .env"))?
            .parse()
            .map_err(|e| anyhow!("invalid SNIFFER_INTERFACE: {e}"))?,
        tcp_stream_ttl: chrono::Duration::seconds(
            dotenv::var("SNIFFER_TCP_STREAM_TTL")
                .map_err(|_| anyhow!("SNIFFER_TCP_STREAM_TTL not found in .env"))?
                .parse()
                .map_err(|e| anyhow!("invalid SNIFFER_TCP_STREAM_TTL: {e}"))?,
        ),
        max_stream_ttl: chrono::Duration::seconds(
            dotenv::var("SNIFFER_MAX_STREAM_TTL")
                .map_err(|_| anyhow!("SNIFFER_MAX_STREAM_TTL not found in .env"))?
                .parse()
                .map_err(|e| anyhow!("invalid SNIFFER_MAX_STREAM_TTL: {e}"))?,
        ),
    })
}
