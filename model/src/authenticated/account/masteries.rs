use serde::{Deserialize, Serialize};

use crate::{Endpoint, FixedEndpoint};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct AccountMastery {
    pub id: u32,
    pub level: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct MasteryRegionTotal {
    pub region: String,
    pub spent: u32,
    pub earned: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct AccountMasteryPoints {
    pub totals: Vec<MasteryRegionTotal>,
    pub unlocked: Vec<u32>,
}

pub type AccountMasteries = Vec<AccountMastery>;

impl Endpoint for AccountMasteries {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/account/masteries";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl FixedEndpoint for AccountMasteries {}

impl Endpoint for AccountMasteryPoints {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/account/mastery/points";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl FixedEndpoint for AccountMasteryPoints {}
