use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub interface: String,
    pub mqtt: MQTTConfig
}

#[derive(Serialize, Deserialize)]
pub struct MQTTConfig {
    pub client_id: String,
    pub host: String,
    pub port: u16,
    pub topic: String
}

impl Default for Config {
    fn default() -> Self {
        Config {
            interface: String::from("wlan0"),
            mqtt: MQTTConfig::default()
        }
    }
}

impl Default for MQTTConfig {
    fn default() -> Self {
        MQTTConfig {
            client_id: String::from("c2-telemetry-proxy"),
            host: String::from("localhost"),
            port: 1883,
            topic: String::from("c2/telemetry")
        }
    }
}