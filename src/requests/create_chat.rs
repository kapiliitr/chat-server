use models::ChatErrorKind::ChatAlreadyExists;
use models::{Chat, ChatError, ChatResult};
use requests::RequestHandler;
use store::{CHATS, USERS};
use uuid::Uuid;

pub struct CreateChatRequest {
    pub participant_ids: Vec<u64>,
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
        let user_id_1 = self.participant_ids[0];
        let user_id_2 = self.participant_ids[1];
        USERS.with(|users| {
            CHATS.with(|chats| {
                if let Some(user_1_chats) = users.borrow().get(&user_id_1) {
                    for chat_id in user_1_chats {
                        if let Some(user_1_chat) = chats.borrow().get(chat_id) {
                            if user_1_chat.0.participant_ids.contains(&user_id_2) {
                                return Some(user_1_chat.0.clone());
                            }
                        }
                    }
                }
                return None;
            })
        })
    }

    fn create_and_get_new_chat(&self) -> Chat {
        let new_chat = Chat {
            id: Uuid::new_v4().as_u128(),
            participant_ids: self.participant_ids.clone(),
        };

        let user_id_1 = self.participant_ids[0];
        let user_id_2 = self.participant_ids[1];

        USERS.with(|users| {
            CHATS.with(|chats| {
                chats
                    .borrow_mut()
                    .insert(new_chat.id, (new_chat.clone(), vec![]));
                {
                    users
                        .borrow_mut()
                        .entry(user_id_1)
                        .and_modify(|user_1_chats| user_1_chats.push(new_chat.id))
                        .or_insert(vec![new_chat.id]);
                }
                {
                    users
                        .borrow_mut()
                        .entry(user_id_2)
                        .and_modify(|user_2_chats| user_2_chats.push(new_chat.id))
                        .or_insert(vec![new_chat.id]);
                }
            })
        });

        new_chat
    }
}
