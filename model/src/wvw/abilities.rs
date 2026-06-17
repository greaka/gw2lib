pub type AbilityId = u32;

use serde::{Deserialize, Serialize};

use crate::{BulkEndpoint, Endpoint, EndpointWithId};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct AbilityRank {
    pub cost: u32,
    pub effect: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Ability {
    pub id: AbilityId,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub ranks: Vec<AbilityRank>,
}

impl Endpoint for Ability {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/wvw/abilities";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for Ability {
    type IdType = AbilityId;
}
impl BulkEndpoint for Ability {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
