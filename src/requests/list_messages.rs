use models::ChatErrorKind::ChatDoesNotExist;
use models::{ChatError, ChatResult, Message};
use requests::RequestHandler;
use store::{CHATS};


pub struct ListMessagesRequest {
    pub chat_id: u128,
}

impl RequestHandler for ListMessagesRequest {
    type Response = Vec<Message>;

    fn execute(self) -> ChatResult<Self::Response> {
        match self.get_messages() {
            Some(messages) => Ok(messages),
            None => Err(ChatError::new(ChatDoesNotExist)),
        }
    }
}

impl ListMessagesRequest {
    fn get_messages(&self) -> Option<Vec<Message>> {
        CHATS.with(|chats| {
            if let Some(chat) = chats.borrow().get(&self.chat_id) {
                return Some(chat.1.clone());
            }
            return None;
        })
    }
}
