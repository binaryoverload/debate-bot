use config::{Config, ConfigError, File};

#[derive(Debug, Deserialize)]
pub struct Discord {
    pub id: u64,
    pub token: String
}

#[derive(Debug, Deserialize)]
pub struct Command {
    pub prefix: String
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub discord: Discord,
    pub command: Command
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut configuration = Config::new();

        configuration.merge(File::with_name("Settings"))?;

        configuration.try_into()
    }
}
