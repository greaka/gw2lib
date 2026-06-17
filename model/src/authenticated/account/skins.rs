use serde::{Deserialize, Serialize};

use crate::{items::skins::SkinId, Endpoint, FixedEndpoint};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AccountSkins(pub Vec<SkinId>);

impl Endpoint for AccountSkins {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/account/skins";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl FixedEndpoint for AccountSkins {}
