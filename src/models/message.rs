use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Message {
    pub id: String,
    pub timestamp: u128,
    pub message: String,
    pub source_user_id: u64,
    pub destination_user_id: u64,
}
