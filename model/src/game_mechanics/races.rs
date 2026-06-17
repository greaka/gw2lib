use serde::{Deserialize, Serialize};

use crate::{game_mechanics::skills::SkillId, BulkEndpoint, Endpoint, EndpointWithId};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Race {
    pub id: String,
    pub name: String,
    pub skills: Vec<SkillId>,
}

impl Endpoint for Race {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/races";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for Race {
    type IdType = String;
}
impl BulkEndpoint for Race {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
