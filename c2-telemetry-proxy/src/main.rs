use std::{process, thread};
use log::{error, info};
use rumqttc::QoS;
use crate::config::Config;
use crate::mqtt::init_mqtt;

mod config;
mod mqtt;

fn main() {
    env_logger::init();
    let config: Config = match confy::load_path("./config.toml") {
        Ok(config) => config,
        Err(error) => {
            error!("{}", error);
            process::exit(2);
        }
    };
    info!("Initializing MQTT");
    let mut client = init_mqtt(
        config.mqtt.client_id,
        config.mqtt.host,
        config.mqtt.port,
    );

    info!("Initializing packet capture");
    let device = pcap::Device::from(config.interface.as_str());
    let mut cap = pcap::Capture::from_device(device)
        .unwrap()
        .promisc(true)
        .open()
        .unwrap();
    for i in 1..200 {
        match client.publish(config.mqtt.topic.clone(), QoS::AtMostOnce, false, "Test Payload") {
            Err(error) => {
                error!("{}", error)
            },
            _ => {}
        }
    }

}
