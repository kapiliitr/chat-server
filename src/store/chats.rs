use models::{Chat, Message};
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
/// Store containing the chats and their messages
pub static CHATS: RefCell<HashMap<u128, (Chat, Vec<Message>)>> = RefCell::new(HashMap::new());
}

// TODO: Define a trait with get/set methods for accessing the storage.
