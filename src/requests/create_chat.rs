use models::ChatErrorKind::ChatAlreadyExists;
use models::{Chat, ChatError, ChatResult};
use requests::RequestHandler;
use store::{CHATS, USERS};

pub struct CreateChatRequest {
    pub chat: Chat,
}

impl RequestHandler for CreateChatRequest {
    type Response = Chat;

    fn execute(self) -> ChatResult<Self::Response> {
        match self.get_chat_if_already_exists() {
            None => Ok(self.create_and_get_new_chat()),
            Some(_chat) => Err(ChatError::new(ChatAlreadyExists)),
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
        CHATS.with(|chats| {
            chats
                .borrow_mut()
                .insert(self.chat.id, (self.chat.clone(), vec![]));
        });

        USERS.with(|users| {
            let user_id_1 = self.chat.participant_ids[0];
            let user_id_2 = self.chat.participant_ids[1];

            let mut users_mut = users.borrow_mut();

            users_mut
                .entry(user_id_1)
                .and_modify(|user_1_chats| user_1_chats.push(self.chat.id))
                .or_insert_with(|| vec![self.chat.id]);

            users_mut
                .entry(user_id_2)
                .and_modify(|user_2_chats| user_2_chats.push(self.chat.id))
                .or_insert_with(|| vec![self.chat.id]);
        });

        self.chat.clone()
    }
}
