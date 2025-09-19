use serde::{Deserialize, Serialize};

use crate::{BulkEndpoint, Endpoint, EndpointWithId, NoAuthentication};

pub type CurrencyId = u16;

#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Currency {
    pub id: CurrencyId,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub order: u16,
}

impl EndpointWithId for Currency {
    type IdType = CurrencyId;
}
impl Endpoint for Currency {
    type Authenticated = NoAuthentication;

    const LOCALE: bool = true;
    const URL: &'static str = "v2/currencies";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}

impl BulkEndpoint for Currency {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
