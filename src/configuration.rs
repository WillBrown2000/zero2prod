#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}
#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    use std::path::PathBuf;

    // Try to load configuration.yaml from current working directory first
    let cwd_path: PathBuf = std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("configuration.yaml");

    // Fallback: load from the project root (compiled-in) if not found in CWD
    let project_root_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("configuration.yaml");

    let chosen_path = if cwd_path.exists() { cwd_path } else { project_root_path };

    let builder = config::Config::builder()
        .add_source(config::File::from(chosen_path).format(config::FileFormat::Yaml))
        // Allow environment overrides using APP__X__Y style, e.g. APP__APPLICATION_PORT, APP__DATABASE__HOST
        .add_source(config::Environment::with_prefix("APP").separator("__"));

    let settings: Settings = builder.build()?.try_deserialize()?;

    Ok(settings)
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}
