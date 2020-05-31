use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Chat {
    pub id: u128,
    pub participant_ids: Vec<u64>,
}
