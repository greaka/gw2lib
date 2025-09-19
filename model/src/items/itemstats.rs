use serde::{Deserialize, Serialize};

use crate::{BulkEndpoint, Endpoint, EndpointWithId, NoAuthentication, items::AttributeType};

pub type StatsId = u32;

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Attribute {
    pub attribute: AttributeType,
    pub multiplier: f32,
    pub value: u8,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct ItemStat {
    pub id: StatsId,
    /// The name of the set of stats. Can be empty.
    pub name: String,
    pub attributes: Vec<Attribute>,
}

impl EndpointWithId for ItemStat {
    type IdType = StatsId;
}

impl Endpoint for ItemStat {
    type Authenticated = NoAuthentication;

    const LOCALE: bool = true;
    const URL: &'static str = "v2/itemstats";
    const VERSION: &'static str = "2023-03-20T19:00:00.000Z";
}

impl BulkEndpoint for ItemStat {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
