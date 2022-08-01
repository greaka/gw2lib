use serde::{Deserialize, Serialize};

use crate::{items::ItemId, Endpoint, FixedEndpoint};

pub type AccountMaterials = Vec<AccountMaterial>;

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct AccountMaterial {
    pub id: ItemId,
    pub category: u32,
    pub count: u32,
}

impl Endpoint for AccountMaterials {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/account/materials";
    const VERSION: &'static str = "2022-07-25T00:00:00.000Z";
}

impl FixedEndpoint for AccountMaterials {}
