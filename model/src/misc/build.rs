use serde::{Deserialize, Serialize};

use crate::{Endpoint, FixedEndpoint};

#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Build {
    pub id: u64,
}

impl Endpoint for Build {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/build";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}

impl FixedEndpoint for Build {}
