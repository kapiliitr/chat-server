use models::ChatErrorKind::NoChatsFound;
use models::{Chat, ChatError, ChatResult};
use requests::RequestHandler;
use store::{CHATS, USERS};


pub struct ListChatsRequest {
    pub user_id: u64,
}

impl RequestHandler for ListChatsRequest {
    type Response = Vec<Chat>;

    fn execute(self) -> ChatResult<Self::Response> {
        match self.get_chats() {
            Some(chats) => Ok(chats),
            None => Err(ChatError::new(NoChatsFound)),
        }
    }
}

impl ListChatsRequest {
    fn get_chats(&self) -> Option<Vec<Chat>> {
        USERS.with(|users| {
            CHATS.with(|chats| {
                if let Some(user_chats) = users.borrow().get(&self.user_id) {
                    let mut found_chats = vec![];
                    for chat_id in user_chats {
                        if let Some(user_chat) = chats.borrow().get(chat_id) {
                            found_chats.push(user_chat.0.clone());
                        }
                    }
                    return Some(found_chats);
                }
                return None;
            })
        })
    }
}
