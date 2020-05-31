#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub id: String,
    pub timestamp: u128,
    pub message: String,
    pub source_user_id: u64,
    pub destination_user_id: u64,
}
