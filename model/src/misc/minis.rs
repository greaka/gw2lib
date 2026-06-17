pub type MiniPetId = u64;

use serde::{Deserialize, Serialize};

use crate::{items::ItemId, BulkEndpoint, Endpoint, EndpointWithId};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Mini {
    pub id: MiniPetId,
    pub name: String,
    pub icon: String,
    pub order: u32,
    pub item_id: ItemId,
}

impl Endpoint for Mini {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/minis";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for Mini {
    type IdType = MiniPetId;
}
impl BulkEndpoint for Mini {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
