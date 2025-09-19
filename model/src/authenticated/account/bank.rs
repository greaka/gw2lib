use crate::{Authenticated, Endpoint, FixedEndpoint, authenticated::characters::InventoryItem};

pub type Bank = Vec<Option<InventoryItem>>;

impl Endpoint for Bank {
    type Authenticated = Authenticated;

    const LOCALE: bool = false;
    const URL: &'static str = "v2/account/bank";
    const VERSION: &'static str = "2022-07-25T00:00:00.000Z";
}

impl FixedEndpoint for Bank {}
