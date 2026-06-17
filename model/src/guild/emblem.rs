use serde::{Deserialize, Serialize};

use crate::{BulkEndpoint, Endpoint, EndpointWithId};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct EmblemBackground {
    pub id: u32,
    pub layers: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct EmblemForeground {
    pub id: u32,
    pub layers: Vec<String>,
}

impl Endpoint for EmblemBackground {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/emblem/backgrounds";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for EmblemBackground {
    type IdType = u32;
}
impl BulkEndpoint for EmblemBackground {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}

impl Endpoint for EmblemForeground {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/emblem/foregrounds";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for EmblemForeground {
    type IdType = u32;
}
impl BulkEndpoint for EmblemForeground {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
