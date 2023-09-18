use serde::{Deserialize, Serialize};

use crate::{items::ItemId, Endpoint, FixedEndpoint};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DeliveryItem {
    pub id: ItemId,
    pub count: u32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Delivery {
    pub coins: u32,
    pub items: Vec<DeliveryItem>,
}

impl Endpoint for Delivery {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/commerce/delivery";
    const VERSION: &'static str = "2023-07-01T00:00:00.000Z";
}

impl FixedEndpoint for Delivery {}
