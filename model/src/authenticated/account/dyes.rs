use serde::{Deserialize, Serialize};

use crate::{misc::colors::ColorId, Endpoint, FixedEndpoint};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AccountDyes(pub Vec<ColorId>);

impl Endpoint for AccountDyes {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/account/dyes";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl FixedEndpoint for AccountDyes {}
