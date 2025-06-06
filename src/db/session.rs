use scylla::client::session::Session;
use scylla::client::session_builder::SessionBuilder;
use anyhow::Result;

pub struct ScyllaSession {
    pub session: Session,
}

impl ScyllaSession {
    pub async fn new() -> Result<Self> {

        let session = SessionBuilder::new()
            .known_node("localhost:9042")
            .known_node("localhost:9043")
            .known_node("localhost:9044")
            .known_node("localhost:9045")
            .build()
            .await?;

        session.query_iter("CREATE KEYSPACE IF NOT EXISTS counting WITH REPLICATION = { 'class' : 'SimpleStrategy', 'replication_factor' : 4 };", &[]).await?;
        session.query_iter("CREATE KEYSPACE IF NOT EXISTS logging WITH REPLICATION = { 'class' : 'SimpleStrategy', 'replication_factor' : 4 };", &[]).await?;
        

        session.query_iter("CREATE TABLE IF NOT EXISTS counting.viewers (
        channel text PRIMARY KEY, 
        viewers counter
        );", &[]).await?;

        session.query_iter("CREATE TABLE IF NOT EXISTS logging.users (
        channel text,
        uid text,
        present boolean,
        PRIMARY KEY (channel, uid)
        );", &[]).await?;

        Ok(Self { session })
    }

}