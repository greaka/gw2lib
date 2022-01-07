use serde::{Deserialize, Serialize};

use crate::{BulkEndpoint, Endpoint};

pub type CurrencyId = u64;

#[derive(Debug, Serialize, Deserialize)]
pub struct Currency {
    pub id:          CurrencyId,
    pub name:        String,
    pub description: String,
    pub icon:        String,
    pub order:       u8,
}

impl Endpoint for Currency {
    fn url() -> &'static str {
        "v2/currencies"
    }
}

impl BulkEndpoint for Currency {
    type IdType = CurrencyId;

    const ALL: bool = true;
    const PAGING: bool = true;
}
