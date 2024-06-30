use crate::entities::message_entity::MessageEntity;

pub struct MessageService;

impl MessageService {
    pub fn get_latest() -> MessageEntity {
        let message = MessageEntity {
            sender: None,
            content: Some(String::from("value")),
        };

        message
    }
}
