use rumqttc::{Client, Event, MqttOptions, Packet, QoS};
use std::time::Duration;

pub struct MqttListener {
    broker: String,
    port: u16,
    client_id: String,
    topic: String,
}

impl MqttListener {
    pub fn new(broker: String, port: u16, client_id: String, topic: String) -> Self {
        Self {
            broker,
            port,
            client_id,
            topic,
        }
    }

    pub fn run<F>(&self, mut handler: F) -> Result<(), rumqttc::ClientError>
    where
        F: FnMut(Vec<u8>),
    {
        let mut opts = MqttOptions::new(&self.client_id, &self.broker, self.port);
        opts.set_keep_alive(Duration::from_secs(30));
        let (mut client, mut connection) = Client::new(opts, 10);
        client
            .subscribe(&self.topic, QoS::AtLeastOnce)
            .expect("failed to subscribe");

        loop {
            match connection.eventloop.poll() {
                Ok(Event::Incoming(Packet::Publish(publish))) => {
                    handler(publish.payload.to_vec());
                }
                Ok(_) => {}
                Err(err) => return Err(err),
            }
        }
    }
}
