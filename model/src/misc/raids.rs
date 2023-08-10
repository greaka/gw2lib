use serde::{Deserialize, Serialize};

use crate::{BulkEndpoint, Endpoint, EndpointWithId};

pub type RaidId = String;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Raid {
    pub id: String,
    pub wings: Vec<Wing>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wing {
    pub id: String,
    pub events: Vec<Event>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

impl EndpointWithId for Raid {
    type IdType = RaidId;
}

impl Endpoint for Raid {
    const AUTHENTICATED: bool = false;
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
