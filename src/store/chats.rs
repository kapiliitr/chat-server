use models::{Chat, Message};
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
pub static CHATS: RefCell<HashMap<u128, (Chat, Vec<Message>)>> = RefCell::new(HashMap::new());
}
