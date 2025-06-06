use crate::repositories::MessageRepo;
use crate::kafka::producer::EventProducer;
use crate::model::message::{ Event, Message, Payload, JoinedUser };
use scylla::{
    client::session::Session,
};
use tokio::time::{interval, Duration};

pub struct CountService {
    session: Session,
    repo: MessageRepo
}

impl CountService {
    fn new(session: Session, repo: MessageRepo) -> Self {
        Self { session, repo }
    }

    async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        
        let _ = tokio::spawn({
            let session = self.session.clone();
            let repo = self.repo.clone();
            async move {
                let mut ticker = interval(Duration::from_secs(1));
                loop {
                    ticker.tick().await;
                    
                }
            }
        });
        Ok(())
    }

}