#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}
#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {

    let base_path = std::env::current_dir().expect("Failed to get current directory");
    let configuration_directory = base_path.join("src/configuration");

    // Determine the environment (defaults to Local when APP_ENVIRONMENT is not set)
    let environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".to_string());
    let environment: Environment = environment
        .try_into()
        .unwrap_or(Environment::Local);

    let environment_filename = format!("{}.yaml", environment.as_str());

    // Build configuration from the files in src/configuration
    let settings = config::Config::builder()
        .add_source(config::File::from(configuration_directory.join("base.yaml")))
        .add_source(config::File::from(configuration_directory.join(environment_filename)))
        .build()?;

    settings.try_deserialize::<Settings>()
}
pub enum Environment {
    Local,
    Production
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Production => "production",
            Environment::Local => "local"
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "production" => Ok(Self::Production),
            "local" => Ok(Self::Local),
            _ => Err(format!("Invalid environment: {}", value))
        }
    }
}
impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}
