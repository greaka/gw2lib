pub type TitleId = u16;

use serde::{Deserialize, Serialize};

use crate::{BulkEndpoint, Endpoint, EndpointWithId};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Title {
    pub id: TitleId,
    pub name: String,
    pub achievement: Option<u32>,
    pub achievements: Vec<u32>,
}

impl Endpoint for Title {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/titles";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for Title {
    type IdType = TitleId;
}
impl BulkEndpoint for Title {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
