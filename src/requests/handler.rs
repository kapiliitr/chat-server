use models::ChatResult;
use requests::{AddMessageRequest, CreateChatRequest, ListChatsRequest, ListMessagesRequest};

pub enum Request {
    CreateChat(CreateChatRequest),
    AddMessage(AddMessageRequest),
    ListChats(ListChatsRequest),
    ListMessages(ListMessagesRequest),
    Invalid,
}

pub trait RequestHandler {
    type Response;
    fn execute(self) -> ChatResult<Self::Response>;
}
