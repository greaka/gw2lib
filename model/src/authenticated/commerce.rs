use serde::{Deserialize, Serialize};

use crate::{items::ItemId, Endpoint, FixedEndpoint, TimeStamp};
pub mod delivery;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Transaction {
    pub id: u64,
    pub item_id: ItemId,
    pub price: u64,
    pub quantity: u32,
    pub created: TimeStamp,
    pub purchased: Option<TimeStamp>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CurrentBuyTransactions(pub Vec<Transaction>);
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CurrentSellTransactions(pub Vec<Transaction>);
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct HistoryBuyTransactions(pub Vec<Transaction>);
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct HistorySellTransactions(pub Vec<Transaction>);

impl Endpoint for CurrentBuyTransactions {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/commerce/transactions/current/buys";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl FixedEndpoint for CurrentBuyTransactions {}

impl Endpoint for CurrentSellTransactions {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/commerce/transactions/current/sells";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl FixedEndpoint for CurrentSellTransactions {}

impl Endpoint for HistoryBuyTransactions {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/commerce/transactions/history/buys";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl FixedEndpoint for HistoryBuyTransactions {}

impl Endpoint for HistorySellTransactions {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/commerce/transactions/history/sells";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl FixedEndpoint for HistorySellTransactions {}
