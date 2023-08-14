use serde::{Deserialize, Serialize};

use crate::{BulkEndpoint, Endpoint, EndpointWithId};

pub type CatId = u32;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Cat {
    pub id: CatId,
    pub hint: String,
}

impl EndpointWithId for Cat {
    type IdType = CatId;
}
impl Endpoint for Cat {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/home/cats";
    const VERSION: &'static str = "2023-08-14T00:00:00.000Z";
}

impl BulkEndpoint for Cat {
    const ALL: bool = false;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
