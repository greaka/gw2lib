pub type AmuletId = u16;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{items::AttributeType, BulkEndpoint, Endpoint, EndpointWithId};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Amulet {
    pub id: AmuletId,
    pub name: String,
    pub icon: String,
    pub attributes: HashMap<AttributeType, u32>,
}

impl Endpoint for Amulet {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/pvp/amulets";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for Amulet {
    type IdType = AmuletId;
}
impl BulkEndpoint for Amulet {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
