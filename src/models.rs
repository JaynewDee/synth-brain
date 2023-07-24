use serde::{Deserialize, Serialize};

pub type ResponseData = Vec<DataItem>;

#[derive(Deserialize, Serialize, Debug)]
pub struct ImageResponse {
    created: i32,
    pub data: ResponseData,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DataItem {
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TextResponse {
    id: String,
    object: String,
    created: u32,
    model: String,
    pub choices: Vec<CompletionChoice>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CompletionChoice {
    pub message: MessageItem,
    finish_reason: String,
    index: i16,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MessageItem {
    pub role: String,
    pub content: String,
}

impl MessageItem {
    pub fn new(role: String, content: String) -> Self {
        Self { role, content }
    }
}
