use std::env;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub app_name: String,
    pub app_env: String,
    pub host: String,
    pub port: u16,
    pub cors_allowed_origin: String,
    pub ollama_base_url: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, String> {
        let app_name = env::var("APP_NAME").unwrap_or_else(|_| "octopus-backend".to_string());
        let app_env = env::var("APP_ENV").unwrap_or_else(|_| "local".to_string());
        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8000".to_string())
            .parse::<u16>()
            .map_err(|_| "PORT must be a valid u16 integer".to_string())?;
        let cors_allowed_origin = env::var("CORS_ALLOWED_ORIGIN")
            .unwrap_or_else(|_| "http://localhost:3000,http://127.0.0.1:3000".to_string());
        let ollama_base_url =
            env::var("OLLAMA_BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:11434".to_string());

        Ok(Self {
            app_name,
            app_env,
            host,
            port,
            cors_allowed_origin,
            ollama_base_url,
        })
    }
}
