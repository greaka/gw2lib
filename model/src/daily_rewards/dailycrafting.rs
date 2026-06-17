use serde::{Deserialize, Serialize};

use crate::{BulkEndpoint, Endpoint, EndpointWithId};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct DailyCrafting {
    pub id: String,
}

impl Endpoint for DailyCrafting {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/dailycrafting";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for DailyCrafting {
    type IdType = String;
}
impl BulkEndpoint for DailyCrafting {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
