#[derive(serde::Deserialize)]
pub struct Setting {
    pub database: DatabaseSettings,
    pub application_port: u16

}
#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String
}
