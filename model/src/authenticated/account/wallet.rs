use serde::{Deserialize, Serialize};

use crate::{misc::currencies::CurrencyId, Endpoint, FixedEndpoint};

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct WalletItem {
    pub id: CurrencyId,
    pub value: u32,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Wallet(pub Vec<WalletItem>);

impl Endpoint for Wallet {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/account/wallet";
    const VERSION: &'static str = "2022-07-25T00:00:00.000Z";
}

impl FixedEndpoint for Wallet {}
