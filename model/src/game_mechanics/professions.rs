use serde::{Deserialize, Serialize};

use crate::{
    game_mechanics::specializations::SpecializationId, BulkEndpoint, Endpoint, EndpointWithId,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct ProfessionInfo {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub icon_big: String,
    pub specializations: Vec<SpecializationId>,
    pub weapons: Option<serde_json::Value>,
    pub flags: Option<Vec<String>>,
    pub skills: Option<Vec<serde_json::Value>>,
    pub training: Option<Vec<serde_json::Value>>,
    pub skills_by_palette: Option<Vec<serde_json::Value>>,
}

impl Endpoint for ProfessionInfo {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/professions";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for ProfessionInfo {
    type IdType = String;
}
impl BulkEndpoint for ProfessionInfo {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
