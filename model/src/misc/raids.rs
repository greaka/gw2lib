use serde::{Deserialize, Serialize};

use crate::{BulkEndpoint, Endpoint, EndpointWithId, NoAuthentication};

pub type RaidId = String;
pub type WingId = String;
pub type EventId = String;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Raid {
    pub id: RaidId,
    pub wings: Vec<Wing>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wing {
    pub id: WingId,
    pub events: Vec<Event>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub id: EventId,
    #[serde(rename = "type")]
    pub _type: String,
}

impl EndpointWithId for Raid {
    type IdType = RaidId;
}

impl Endpoint for Raid {
    type Authenticated = NoAuthentication;

    const LOCALE: bool = true;
    const URL: &'static str = "v2/raids";
    const VERSION: &'static str = "2023-08-02T00:00:00.000Z";
}

impl BulkEndpoint for Raid {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
