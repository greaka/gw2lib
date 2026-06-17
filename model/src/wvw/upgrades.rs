use serde::{Deserialize, Serialize};

use crate::{BulkEndpoint, Endpoint, EndpointWithId};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct WvwUpgradeDetail {
    pub name: String,
    pub description: String,
    pub icon: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct WvwUpgradeTier {
    pub name: String,
    pub yaks_required: u32,
    pub upgrades: Vec<WvwUpgradeDetail>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct WvwUpgrade {
    pub id: u32,
    pub tiers: Vec<WvwUpgradeTier>,
}

impl Endpoint for WvwUpgrade {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/wvw/upgrades";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for WvwUpgrade {
    type IdType = u32;
}
impl BulkEndpoint for WvwUpgrade {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
