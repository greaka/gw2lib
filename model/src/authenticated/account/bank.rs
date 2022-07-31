use serde::{Deserialize, Serialize};

use crate::{authenticated::characters::InventoryItem, Endpoint, FixedEndpoint};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq)]
#[serde(transparent)]
pub struct Bank(pub Vec<Option<InventoryItem>>);

impl Endpoint for Bank {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/account/bank";
    const VERSION: &'static str = "2022-07-25T00:00:00.000Z";
}

impl FixedEndpoint for Bank {}
