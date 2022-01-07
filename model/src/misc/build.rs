use serde::{Deserialize, Serialize};

use crate::{Endpoint, FixedEndpoint};

#[derive(Debug, Serialize, Deserialize)]
pub struct Build {
    pub id: u64,
}

impl Endpoint for Build {
    fn url() -> &'static str {
        "v2/build"
    }
}

impl FixedEndpoint for Build {}
