use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Incoming {
    BecomeCaptain,
    LeaveCrew,
    JoinCrew {
        id: Uuid,
    },
}
