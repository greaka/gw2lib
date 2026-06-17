use serde::{Deserialize, Serialize};

use crate::{misc::minis::MiniPetId, Endpoint, FixedEndpoint};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AccountMinis(pub Vec<MiniPetId>);

impl Endpoint for AccountMinis {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/account/minis";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl FixedEndpoint for AccountMinis {}
