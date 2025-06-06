
use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    ClientConfig,
};
use crate::model::message::{ Event };

pub struct EventProducer {
    producer: FutureProducer,
    topic: String,
}

impl EventProducer {
    pub fn new(broker: &str, topic: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", broker)
            .create()?;

        Ok(EventProducer {
            producer,
            topic: topic.to_string(),
        })
    }

    pub async fn post_message(&self, payload: Event) -> Result<(), Box<dyn std::error::Error>> {
        let message = serde_json::to_string(&payload)?;
        self.producer
            .send(
                FutureRecord::<(), _>::to(&self.topic)
                    .payload(&message),
                rdkafka::util::Timeout::Never,
            )
            .await
            .expect("Failed to send message");
        Ok(())
    }
}
