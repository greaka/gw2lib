use serde::{Deserialize, Serialize};

use crate::{BulkEndpoint, Endpoint, EndpointWithId};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct DungeonPath {
    pub id: String,
    #[serde(rename = "type")]
    pub _type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Dungeon {
    pub id: String,
    pub paths: Vec<DungeonPath>,
}

impl Endpoint for Dungeon {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/dungeons";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for Dungeon {
    type IdType = String;
}
impl BulkEndpoint for Dungeon {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
