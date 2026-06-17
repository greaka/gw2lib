use serde::{Deserialize, Serialize};

use crate::{BulkEndpoint, Endpoint, EndpointWithId};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Quaggan {
    pub id: String,
    pub url: String,
}

impl Endpoint for Quaggan {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/quaggans";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for Quaggan {
    type IdType = String;
}
impl BulkEndpoint for Quaggan {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
