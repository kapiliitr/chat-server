use mio::net::TcpStream;
use mio::Token;
use models::ChatResult;
use requests;
use requests::{Request, RequestHandler};
use serde::Serialize;
use server::{create_error_response, create_success_response, invalid_response, Server};
use std::io::Read;
use std::net::SocketAddr;
use std::time::Duration;
use std::{io, thread};

pub struct Connection {
    pub socket: TcpStream,
    pub token: Token,
    pub address: SocketAddr,
    pub pending: Option<Vec<String>>,
}

impl Connection {
    pub fn process_request(&mut self, request_string: &String) {
        let mut headers = [httparse::EMPTY_HEADER; 0];
        let mut req = httparse::Request::new(&mut headers);
        let res = req.parse(request_string.as_bytes());
        debug!("Received request {:?}, {:?}", req, res);

        let method = req.method.clone().unwrap();
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
