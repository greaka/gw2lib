use serde::{Deserialize, Serialize};

use crate::{BulkEndpoint, Endpoint, EndpointWithId};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct MapChest {
    pub id: String,
}

impl Endpoint for MapChest {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/mapchests";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for MapChest {
    type IdType = String;
}
impl BulkEndpoint for MapChest {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
