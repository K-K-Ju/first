use config::Config;

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u64,
    pub host: String,
    pub database_name: String
}

impl ToString for DatabaseSettings{
    fn to_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database_url: DatabaseSettings,
    pub app_port: u16
}

impl TryFrom<Config> for Settings {
    type Error = config::ConfigError;

    fn try_from(value: Config) -> Result<Self, Self::Error> {
        let db_map = value.get_table("database")?;

        let db_s = DatabaseSettings {
            database_name: db_map.get("database_name").unwrap().to_string(),
            username: db_map.get("username").unwrap().to_string(),
            password: db_map.get("password").unwrap().to_string(),
            port: db_map.get("port").unwrap().clone().into_uint().unwrap(),
            host: db_map.get("host").unwrap().to_string(),
        };

        Ok(Self {app_port : value.get("application_port")?, database_url: db_s})
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let config=  config::Config::builder()
        .add_source(config::File::with_name("configuration"))
        .build().expect("Can't read config file");

    config.try_into()
}
