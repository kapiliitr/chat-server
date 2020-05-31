use models::{Chat, Message};
use requests;
use requests::{AddMessageRequest, CreateChatRequest, ListChatsRequest, ListMessagesRequest};
use std::str::FromStr;

#[derive(Debug)]
pub struct HttpRequest<'a> {
    pub method: &'a str,
    pub path: &'a str,
    pub body: &'a str,
}

impl<'a> From<HttpRequest<'a>> for requests::Request {
    fn from(req: HttpRequest<'a>) -> Self {
        match req.method {
            "GET" => {
                let parts: Vec<&str> = req.path.split("/").collect();
                if parts.len() == 2 && parts[1].starts_with("chats?userId=") {
                    let user_id_str: String =
                        parts[1].chars().skip("chats?userId=".len()).collect();
                    if let Ok(user_id) = u64::from_str(&user_id_str) {
                        return requests::Request::ListChats(ListChatsRequest { user_id });
                    } else {
                        return requests::Request::Invalid;
                    }
                } else if parts.len() == 4 && parts[1] == "chats" && parts[3] == "messages" {
                    if let Ok(chat_id) = u128::from_str(parts[2]) {
                        return requests::Request::ListMessages(ListMessagesRequest { chat_id });
                    } else {
                        return requests::Request::Invalid;
                    }
                }
                requests::Request::Invalid
            }
            "POST" => {
                let parts: Vec<&str> = req.path.split("/").collect();
                if parts.len() == 2 && parts[1] == "chats" {
                    let new_chat = match serde_json::from_str::<Chat>(req.body) {
                        Ok(r) => r,
                        Err(err) => {
                            warn!("Error parsing request body {}", err);
                            return requests::Request::Invalid;
                        }
                    };
                    return requests::Request::CreateChat(CreateChatRequest { chat: new_chat });
                } else if parts.len() == 4 && parts[1] == "chats" && parts[3] == "messages" {
                    let new_message = match serde_json::from_str::<Message>(req.body) {
                        Ok(r) => r,
                        Err(err) => {
                            warn!("Error parsing request body {}", err);
                            return requests::Request::Invalid;
                        }
                    };
                    if let Ok(chat_id) = u128::from_str(parts[2]) {
                        return requests::Request::AddMessage(AddMessageRequest {
                            chat_id,
                            message: new_message,
                        });
                    } else {
                        return requests::Request::Invalid;
                    }
                }
                requests::Request::Invalid
            }
            _ => return requests::Request::Invalid,
        }
    }
}
