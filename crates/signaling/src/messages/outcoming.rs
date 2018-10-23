use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Outcoming {
    AlreadyCrewed,
    NotCrewed,
    NewCaptain {
        id: Uuid,
    },
    LeaveCrew,
}
