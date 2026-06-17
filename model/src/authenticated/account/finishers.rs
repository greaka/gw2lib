use serde::{Deserialize, Serialize};

use crate::{Endpoint, FixedEndpoint};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct AccountFinisher {
    pub id: u32,
    pub permanent: bool,
    pub quantity: Option<u32>,
}

pub type AccountFinishers = Vec<AccountFinisher>;

impl Endpoint for AccountFinishers {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/account/finishers";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl FixedEndpoint for AccountFinishers {}
