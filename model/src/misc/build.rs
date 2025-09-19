use serde::{Deserialize, Serialize};

use crate::{Endpoint, FixedEndpoint, NoAuthentication};

#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Build {
    pub id: u64,
}

impl Endpoint for Build {
    type Authenticated = NoAuthentication;

    const LOCALE: bool = false;
    const URL: &'static str = "v2/build";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}

impl FixedEndpoint for Build {}
