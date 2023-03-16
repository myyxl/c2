use std::thread;
use std::time::Duration;
use log::{debug, error};
use rumqttc::{Client, Connection, MqttOptions, QoS};

pub fn init_mqtt(
    client_id: String,
    host: String,
    port: u16
) -> Client {
    let mut mqtt_options = MqttOptions::new(client_id, host, port);
    mqtt_options.set_keep_alive(Duration::from_secs(5));
    let (client, mut connection) = Client::new(mqtt_options, 10);
    thread::spawn(move || {
        connection.iter().for_each(|_| {});
    });
    client
}