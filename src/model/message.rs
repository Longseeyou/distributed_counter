use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct Message {
    pub msg: Option<String>,

    #[serde(rename = "type")]
    pub _type: Option<String>,

    pub user_id: Option<String>,
    pub user_name: Option<String>,
    pub avatar: Option<String>,
    pub msg_uid: Option<String>,
    pub role: Option<String>,
    pub date_create: Option<String>,
    pub reply: Option<String>,
    pub sticker: Option<String>,
    pub device_id: Option<String>,
    pub duration: Option<i32>,
    pub is_verify: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct Payload {
    pub sent_msg: Message,
    pub channel: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PostResponse {
    pub result: bool,
}

#[derive(Serialize, Deserialize)]
pub struct GetViewResponse {
    pub viewers: i64,
}

#[derive(Serialize, Deserialize)]
pub struct GetMessageResponse {
    pub msgs: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Event {
    pub join: Option<bool>,
    pub payload: Option<Payload>,
}

#[derive(Serialize, Deserialize)]
pub struct JoinedUser {
    pub uid: String,
    pub channel: String,
    
}
