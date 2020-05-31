use models::ChatErrorKind::ChatDoesNotExist;
use models::{ChatError, ChatResult, Message};
use requests::RequestHandler;
use std::time::SystemTime;
use store::{CHATS, USERS};
use uuid::Uuid;

pub struct AddMessageRequest {
    pub chat_id: u128,
    pub message: String,
    pub source_user_id: u64,
    pub destination_user_id: u64,
}

impl RequestHandler for AddMessageRequest {
    type Response = Message;

    fn execute(self) -> ChatResult<Self::Response> {
        match self.add_message_if_chat_exists() {
            Some(message) => Ok(message),
            None => Err(ChatError::new(ChatDoesNotExist)),
        }
    }
}

impl AddMessageRequest {
    fn add_message_if_chat_exists(self) -> Option<Message> {
        CHATS.with(|chats| {
            if let Some(chat) = chats.borrow_mut().get_mut(&self.chat_id) {
                let current_timestamp =
                    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
                        Ok(dur) => dur.as_millis(),
                        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
                    };

                let new_message = Message {
                    id: Uuid::new_v4().to_string(),
                    timestamp: current_timestamp,
                    message: self.message,
                    source_user_id: self.source_user_id,
                    destination_user_id: self.destination_user_id,
                };

                chat.1.push(new_message.clone());
                return Some(new_message);
            }
            return None;
        })
    }
}
