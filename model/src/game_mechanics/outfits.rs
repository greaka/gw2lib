use serde::{Deserialize, Serialize};

use crate::{items::ItemId, BulkEndpoint, Endpoint, EndpointWithId};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Outfit {
    pub id: u32,
    pub name: String,
    pub icon: String,
    pub unlock_items: Vec<ItemId>,
}

impl Endpoint for Outfit {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/outfits";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for Outfit {
    type IdType = u32;
}
impl BulkEndpoint for Outfit {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
