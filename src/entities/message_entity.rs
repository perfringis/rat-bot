
use serde::Serialize;

#[derive(Serialize)]
pub struct MessageEntity {
    pub sender: Option<String>,
    pub content: Option<String>,
}
