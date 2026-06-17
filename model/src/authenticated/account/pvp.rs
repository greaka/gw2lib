use serde::{Deserialize, Serialize};

use crate::{Endpoint, FixedEndpoint};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AccountPvpHeroes(pub Vec<u32>);

impl Endpoint for AccountPvpHeroes {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/account/pvp/heroes";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl FixedEndpoint for AccountPvpHeroes {}
