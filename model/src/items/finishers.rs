use serde::{Deserialize, Serialize};

use crate::{items::ItemId, BulkEndpoint, Endpoint, EndpointWithId};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Finisher {
    pub id: u32,
    pub name: String,
    pub icon: String,
    pub unlock_details: String,
    pub unlock_items: Vec<ItemId>,
    pub order: u32,
}

impl Endpoint for Finisher {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/finishers";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for Finisher {
    type IdType = u32;
}
impl BulkEndpoint for Finisher {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
