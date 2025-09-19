use serde::{Deserialize, Serialize};

use crate::{Authenticated, Endpoint, FixedEndpoint, items::ItemId};

pub type AccountMaterials = Vec<AccountMaterial>;

#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct AccountMaterial {
    pub id: ItemId,
    pub category: u32,
    pub count: u32,
}

impl Endpoint for AccountMaterials {
    type Authenticated = Authenticated;

    const LOCALE: bool = false;
    const URL: &'static str = "v2/account/materials";
    const VERSION: &'static str = "2022-07-25T00:00:00.000Z";
}

impl FixedEndpoint for AccountMaterials {}
