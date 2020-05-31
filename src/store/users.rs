use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
pub static USERS: RefCell<HashMap<u64, Vec<u128>>> = RefCell::new(HashMap::new());
}
