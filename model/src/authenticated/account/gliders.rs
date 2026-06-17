use serde::{Deserialize, Serialize};

use crate::{Endpoint, FixedEndpoint};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AccountGliders(pub Vec<u32>);

impl Endpoint for AccountGliders {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/account/gliders";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl FixedEndpoint for AccountGliders {}
