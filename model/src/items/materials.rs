use serde::{Deserialize, Serialize};

use crate::{items::ItemId, BulkEndpoint, Endpoint, EndpointWithId};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct MaterialCategory {
    pub id: u32,
    pub name: String,
    pub items: Vec<ItemId>,
    pub order: Option<u32>,
}

impl Endpoint for MaterialCategory {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/materials";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for MaterialCategory {
    type IdType = u32;
}
impl BulkEndpoint for MaterialCategory {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
