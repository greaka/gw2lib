use serde::{Deserialize, Serialize};

use crate::{misc::titles::TitleId, Endpoint, FixedEndpoint};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AccountTitles(pub Vec<TitleId>);

impl Endpoint for AccountTitles {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/account/titles";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl FixedEndpoint for AccountTitles {}
