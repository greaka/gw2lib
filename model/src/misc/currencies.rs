use serde::{Deserialize, Serialize};

use crate::{BulkEndpoint, Endpoint};

pub type CurrencyId = u64;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Currency {
    pub id:          CurrencyId,
    pub name:        String,
    pub description: String,
    pub icon:        String,
    pub order:       u8,
}

impl_id!(Currency, CurrencyId);
impl Endpoint for Currency {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/currencies";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}

impl BulkEndpoint for Currency {
    const ALL: bool = true;
    const PAGING: bool = true;
}
