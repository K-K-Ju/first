use config::Config;

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u64,
    pub host: String,
    pub db_name: String
}

impl DatabaseSettings {
    pub fn to_string_no_db_name(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

impl ToString for DatabaseSettings{
    fn to_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.db_name
        )
    }
}

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub app_port: u16
}

impl TryFrom<Config> for Settings {
    type Error = config::ConfigError;

    fn try_from(value: Config) -> Result<Self, Self::Error> {
        let db_map = value.get_table("database")?;

        let db_s = DatabaseSettings {
            db_name: db_map.get("database_name").unwrap().to_string(),
            username: db_map.get("username").unwrap().to_string(),
            password: db_map.get("password").unwrap().to_string(),
            port: db_map.get("port").unwrap().clone().into_uint()?,
            host: db_map.get("host").unwrap().to_string(),
        };

        Ok(Self {app_port : value.get("application_port")?, database: db_s})
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let config=  config::Config::builder()
        .add_source(config::File::with_name("configuration"))
        .build().expect("Can't read config file");

    config.try_into()
}
