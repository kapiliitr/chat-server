use serde::Serialize;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chat {
    pub id: u128,
    pub participant_ids: Vec<u64>,
}
