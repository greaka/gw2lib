use serde::{Deserialize, Serialize};

use crate::{game_mechanics::skills::SkillId, BulkEndpoint, Endpoint, EndpointWithId};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Legend {
    pub id: String,
    pub swap: SkillId,
    pub heal: SkillId,
    pub elite: SkillId,
    pub utilities: Vec<SkillId>,
}

impl Endpoint for Legend {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/legends";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for Legend {
    type IdType = String;
}
impl BulkEndpoint for Legend {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
