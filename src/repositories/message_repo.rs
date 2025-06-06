use scylla::{
    client::session::Session, 
    statement::prepared, 
    value::{Row, Counter},
    statement::prepared::PreparedStatement
};
use crate::model::message::{JoinedUser};
use serde::{Deserialize, Serialize};
use serde_json;
use anyhow::Result;
use futures::{StreamExt, TryStreamExt};

pub struct MessageRepo {
    pub session: Session,
}

impl MessageRepo {
    pub fn new(session: Session) -> Self {
        Self { session }
    }

    pub async fn user_exists(&self, uid: &str, channel: &str) -> Result<bool> {
        let query = "SELECT uid FROM logging.users WHERE uid = ? AND channel = ?";
        let prepared: PreparedStatement = self.session.prepare(query).await?;
        let mut page = self.session.execute_iter(prepared, (uid, channel)).await?;
        let rows = page.rows_stream::<(String,)>()?;
        let cnt = rows.count().await;
        Ok(cnt > 0)
    }

    pub async fn update_view(&self, payload:JoinedUser, delta: i64) -> Result<()> { 
        let query = "UPDATE counting.viewers SET viewers = viewers + ? WHERE channel = ?";
        let prepared: PreparedStatement = self.session.prepare(query).await?;
        self.session.execute_iter(prepared, (Counter(delta), &payload.channel)).await?;
        Ok(())
    }

    pub async fn get_viewers(&self, channel: String) -> Result<i64> {
        let query = "SELECT viewers FROM counting.viewers WHERE channel = ?";
        let prepared: PreparedStatement = self.session.prepare(query).await?;
        let mut page = self.session.execute_iter(prepared, (channel,)).await?;
        let mut rows = page.rows_stream::<Row>()?;
        if let Some(row) = rows.next().await.transpose()? {
            let viewers: i64 = row.columns[0].as_ref().unwrap().as_counter().unwrap().0;
            Ok(viewers)
        } else {
            Ok(0)
        }
    }

    pub async fn get_viewers_by_channel(&self, channel: String) -> Result<i64> {
        let query = "SELECT uid FROM logging.users WHERE channel = ? AND present = true ALLOW FILTERING";
        let prepared: PreparedStatement = self.session.prepare(query).await?;
        let mut page = self.session.execute_iter(prepared, (channel,)).await?;
        let count: i64 = page.rows_stream::<(String,)>()?.count().await as i64;
        Ok(count)
    }
}