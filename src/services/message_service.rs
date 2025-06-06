use crate::repositories::MessageRepo;
use crate::kafka::producer::EventProducer;
use crate::model::message::{ Event, Message, Payload, JoinedUser };

pub struct MessageService {
    pub repo: MessageRepo,
    pub event: EventProducer,
}

impl MessageService {
    pub fn new(repo: MessageRepo, event: EventProducer) -> Self {
        Self { repo, event }
    }

    pub async fn produce_message(&self, payload: Event) -> Result<(), Box<dyn std::error::Error>> {
        self.event.post_message(payload).await?;
        Ok(())
    }

    pub async fn update_view(&self, payload: Payload, delta: i64) -> Result<(), Box<dyn std::error::Error>> {
        
        let user: JoinedUser = JoinedUser {
            channel: payload.channel.clone().unwrap_or_else(|| String::from("")),
            uid: payload.sent_msg.user_id.clone().unwrap_or_else(|| String::from("")),
        };
        self.repo.update_view(user, delta).await?;
        Ok(())
    }

    pub async fn get_viewers(&self, channel_id: String) -> Result<i64, Box<dyn std::error::Error>> {
        let viewers = self.repo.get_viewers(channel_id).await?;
        Ok(viewers)
    }

}