use std::string::FromUtf8Error;
use std::io::Error;
use toml::de::Error as TomlError;

#[derive(Debug)]
pub enum ConfigError {
    IoError(Error),
    TomlError(TomlError),
    Utf8Error(FromUtf8Error),
}
impl From<Error> for ConfigError {
    fn from(error: Error) -> Self {
        ConfigError::IoError(error)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(error: toml::de::Error) -> Self {
        ConfigError::TomlError(error)
    }
}

impl From<FromUtf8Error> for ConfigError {
    fn from(error: FromUtf8Error) -> Self {
        ConfigError::Utf8Error(error)
    }
}
