use url::Url;

pub struct ParsedConnection {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub user: String,
    pub password: String,
}

pub fn parse_postgres_url(input: &str) -> Result<ParsedConnection, String> {
    let url = Url::parse(input.trim()).map_err(|e| format!("Invalid URL: {}", e))?;

    let scheme = url.scheme();
    if scheme != "postgresql" && scheme != "postgres" {
        return Err("URL scheme must be postgresql:// or postgres://".to_string());
    }

    Ok(ParsedConnection {
        user: url.username().to_string(),
        password: url.password().unwrap_or("").to_string(),
        host: url.host_str().unwrap_or("localhost").to_string(),
        port: url.port().unwrap_or(5432),
        database: url.path().trim_start_matches('/').to_string(),
    })
}

pub fn random_connection_name() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    format!("conn-{}", ts)
}
