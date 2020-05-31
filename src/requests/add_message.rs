use models::ChatErrorKind::ChatDoesNotExist;
use models::{ChatError, ChatResult, Message};
use requests::RequestHandler;

use store::{CHATS};


pub struct AddMessageRequest {
    pub chat_id: u128,
    pub message: Message,
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
                chat.1.push(self.message.clone());
                return Some(self.message);
            }
            return None;
        })
    }
}
