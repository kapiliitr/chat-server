use models::ChatErrorKind::ChatAlreadyExists;
use models::{Chat, ChatError, ChatResult};
use requests::RequestHandler;
use store::{CHATS, USERS};
use uuid::Uuid;

pub struct CreateChatRequest {
    pub chat: Chat,
}

impl RequestHandler for CreateChatRequest {
    type Response = Chat;

    fn execute(self) -> ChatResult<Self::Response> {
        match self.get_chat_if_already_exists() {
            None => Ok(self.create_and_get_new_chat()),
            Some(chat) => Err(ChatError::new(ChatAlreadyExists)),
        }
    }
}

impl CreateChatRequest {
    fn get_chat_if_already_exists(&self) -> Option<Chat> {
        CHATS.with(|chats| {
            if let Some(chat_entry) = chats.borrow().get(&self.chat.id) {
                return Some(chat_entry.0.clone());
            }
            None
        })
    }

    fn create_and_get_new_chat(&self) -> Chat {
        USERS.with(|users| {
            CHATS.with(|chats| {
                let user_id_1 = self.chat.participant_ids[0];
                let user_id_2 = self.chat.participant_ids[1];
                chats
                    .borrow_mut()
                    .insert(self.chat.id, (self.chat.clone(), vec![]));
                {
                    users
                        .borrow_mut()
                        .entry(user_id_1)
                        .and_modify(|user_1_chats| user_1_chats.push(self.chat.id))
                        .or_insert(vec![self.chat.id]);
                }
                {
                    users
                        .borrow_mut()
                        .entry(user_id_2)
                        .and_modify(|user_2_chats| user_2_chats.push(self.chat.id))
                        .or_insert(vec![self.chat.id]);
                }
            })
        });

        self.chat.clone()
    }
}
