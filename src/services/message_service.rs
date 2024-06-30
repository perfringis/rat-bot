use crate::entities::message_entity::MessageEntity;

pub struct MessageService;

impl MessageService {
    pub fn get_latest() -> MessageEntity {
        let message = MessageEntity {
            sender: Some(String::from("value")),
            content: Some(String::from("value")),
        };

        message
    }

    pub fn get_latest_sender() -> MessageEntity {
        let message = MessageEntity {
            sender: Some(String::from("value")),
            content: None,
        };

        message
    }

    pub fn get_latest_content() -> MessageEntity {
        let message = MessageEntity {
            sender: None,
            content: Some(String::from("value")),
        };

        message
    }
}
