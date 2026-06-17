use serde::{Deserialize, Serialize};

use crate::{Endpoint, FixedEndpoint};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AccountMountSkins(pub Vec<u32>);

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AccountMountTypes(pub Vec<String>);

impl Endpoint for AccountMountSkins {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/account/mounts/skins";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl FixedEndpoint for AccountMountSkins {}

impl Endpoint for AccountMountTypes {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/account/mounts/types";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl FixedEndpoint for AccountMountTypes {}
