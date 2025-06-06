use std::sync::Arc;
use dashmap::DashMap;
use rdkafka::{ClientConfig, Message};
use rdkafka::consumer::{Consumer, StreamConsumer};
use actix_web::web::Data;
use crate::model::message::{Event, Payload, JoinedUser};
use crate::services::message_service::MessageService;
use tokio::time::{interval, Duration, timeout};
use tokio::sync::{Mutex, Semaphore};

type UserKey = (String, String);

pub struct EventConsumer {
    consumer: StreamConsumer,
    topic: String,
}

impl EventConsumer {
    pub fn new(broker: &str, topic: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", broker)
            .set("group.id", "view-counter")
            .set("enable.auto.commit", "false")
            .create()?;

        consumer.subscribe(&[topic])?;

        Ok(EventConsumer {
            consumer,
            topic: topic.to_string(),
        })
    }

    pub async fn run(self, service: &Data::<MessageService>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let consumer = self.consumer;
        let service = service.clone();
        let locks: Arc<DashMap<UserKey, Arc<Mutex<()>>>> = Arc::new(DashMap::new());
        let max_threads = 100;
        let semaphore = Arc::new(Semaphore::new(max_threads));

        let _ = tokio::spawn(async move {
            loop {
                if let Ok(message) = consumer.recv().await {
                    let payload = match message.payload() {
                        Some(data) => data,
                        None => {
                            eprintln!("No payload in message");
                            continue;
                        }
                    };

                    let parsed_event: Event = match serde_json::from_slice(payload) {
                        Ok(ev) => ev,
                        Err(e) => {
                            eprintln!("Failed to parse event: {}", e);
                            continue;
                        }
                    };

                    let joined = parsed_event.join.unwrap_or(false);
                    let payload = match parsed_event.payload {
                        Some(p) => p,
                        None => {
                            eprintln!("No payload content");
                            continue;
                        }
                    };

                    let uid = payload.sent_msg.user_id.clone().unwrap_or_default();
                    let channel = payload.channel.clone().unwrap_or_default();

                    let key = (uid, channel);
                    let lock_entry = locks.entry(key).or_insert_with(|| Arc::new(Mutex::new(())));
                    let user_lock = lock_entry.clone();

                    let service = service.clone();
                    let payload = payload.clone();

                    let _permit = semaphore.clone().acquire_owned().await;

                    tokio::spawn(async move {
                        let permit = _permit;
                        let _guard = user_lock.lock().await;

                        let delta = if joined { 1 } else { -1 };
                        if let Err(e) = service.update_view(payload, delta).await {
                            eprintln!("Failed to update view: {}", e);
                        }
                    });
                }
            }
        });
        Ok(())
    }
}
