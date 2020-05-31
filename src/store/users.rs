use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
/// Store containing the list of chats of which each user is a part of
pub static USERS: RefCell<HashMap<u64, Vec<u128>>> = RefCell::new(HashMap::new());
}

// TODO: Define a trait with get/set methods for accessing the storage.
