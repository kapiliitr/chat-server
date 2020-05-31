use core::{create_error_response, create_success_response, invalid_response, HttpRequest};
use mio::net::TcpStream;
use mio::Token;
use models::ChatResult;
use requests;
use requests::{Request, RequestHandler};
use serde::Serialize;

use std::net::SocketAddr;

pub struct Connection {
    pub socket: TcpStream,
    pub token: Token,
    pub address: SocketAddr,
    pub pending: Option<Vec<String>>,
}

impl Connection {
    pub fn process_request(&mut self, request_string: &str) {
        // HTTP requests have the following format-
        //     Method Request-URI HTTP-Version CRLF
        //     headers CRLF
        //     message-body
        // We split the request string line by line.
        let request_parts: Vec<&str> = request_string.split("\r\n").collect();
        // We split the first line by space to extract the HTTP method and path
        let x: Vec<&str> = request_parts[0].split(' ').collect();
        let method: &str = x[0];
        let path: &str = x[1];
        // The last line of the request is the request body if present, otherwise it will be empty string.
        let body: &str = request_parts[request_parts.len() - 1];
        let req = HttpRequest { method, path, body };
        debug!("Received request {:?}", req);

        let method = req.method;
        let chat_request: requests::Request = req.into();
        let response = match chat_request {
            Request::CreateChat(create_chat) => generate_response(method, create_chat.execute()),
            Request::AddMessage(add_message) => generate_response(method, add_message.execute()),
            Request::ListChats(list_chats) => generate_response(method, list_chats.execute()),
            Request::ListMessages(list_messages) => {
                generate_response(method, list_messages.execute())
            }
            Request::Invalid => invalid_response(),
        };
        self.pending = Some(response);
    }
}

fn generate_response<T>(method: &str, result: ChatResult<T>) -> Vec<String>
where
    T: Serialize,
{
    match result {
        Ok(res) => create_success_response(method, res),
        Err(err) => create_error_response(err),
    }
}
