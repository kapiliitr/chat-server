use requests;
use requests::{CreateChatRequest, ListChatsRequest, ListMessagesRequest};
use std::str::FromStr;

impl From<httparse::Request<'_, '_>> for requests::Request {
    fn from(req: httparse::Request) -> Self {
        match req.method {
            Some("GET") => match req.path {
                Some(path) => {
                    let parts: Vec<&str> = path.split("/").collect();
                    info!("Request received {:?}", parts);
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
                            return requests::Request::ListMessages(ListMessagesRequest {
                                chat_id,
                            });
                        } else {
                            return requests::Request::Invalid;
                        }
                    }
                    requests::Request::Invalid
                }
                _ => return requests::Request::Invalid,
            },
            Some("POST") => match req.path {
                //Some(path) => unimplemented!(),
                _ => return requests::Request::Invalid,
            },
            _ => return requests::Request::Invalid,
        }
    }
}
