use serde::{Deserialize, Serialize};

use crate::{BulkEndpoint, Endpoint, EndpointWithId};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct MasteryLevel {
    pub name: String,
    pub description: String,
    pub instruction: String,
    pub icon: String,
    pub point_cost: u32,
    pub exp_cost: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Mastery {
    pub id: u32,
    pub name: String,
    pub requirement: String,
    pub order: u32,
    pub background: String,
    pub region: String,
    pub levels: Vec<MasteryLevel>,
}

impl Endpoint for Mastery {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/masteries";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for Mastery {
    type IdType = u32;
}
impl BulkEndpoint for Mastery {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
