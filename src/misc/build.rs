use crate::utils::*;
use rest_client::*;
use serde::{Deserialize, Serialize};

#[rest(
    "https://api.guildwars2.com/v2/build?v=2019-04-22T00:00:00Z",
    wrapper = "ApiResult"
)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Build {
    pub id: u64,
}

/// ```
/// use gw2api::misc::*;
///
/// get_build().unwrap();
/// ```
pub fn get_build() -> Result<ApiResult<Box<Build>>, Box<dyn std::error::Error>> {
    Build::get(Vec::<bool>::new())
}
