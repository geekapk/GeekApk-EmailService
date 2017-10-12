use std::fs::File;
use std::io::Read;
use std::error::Error;
use serde_json;

#[derive(Clone)]
pub struct Config {
    pub redis_url: String,
    pub redis_prefix: String,
    pub sendgrid_api_key: String,
    pub email_from_address: String
}

#[derive(Deserialize)]
struct RawConfig {
    redis_url: Option<String>,
    redis_prefix: Option<String>,
    sendgrid_api_key: String,
    email_from_address: String
}

impl From<RawConfig> for Config {
    fn from(other: RawConfig) -> Config {
        Config {
            redis_url: other.redis_url.unwrap_or("redis://127.0.0.1/".to_string()),
            redis_prefix: other.redis_prefix.unwrap_or("geekapk_".to_string()),
            sendgrid_api_key: other.sendgrid_api_key,
            email_from_address: other.email_from_address
        }
    }
}

impl Config {
    pub fn load_from_file<T: AsRef<str>>(p: T) -> Result<Config, Box<Error>> {
        let p = p.as_ref();
        let mut f = File::open(p)?;
        let mut content = String::new();
        f.read_to_string(&mut content)?;

        let raw_config: RawConfig = serde_json::from_str(content.as_str())?;
        Ok(raw_config.into())
    }
}
