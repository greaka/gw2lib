pub type TraitId = u16;

use serde::{Deserialize, Serialize};

use crate::{
    game_mechanics::specializations::SpecializationId, BulkEndpoint, Endpoint, EndpointWithId,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Trait {
    pub id: TraitId,
    pub tier: u8,
    pub order: u8,
    pub name: String,
    pub description: Option<String>,
    pub slot: String,
    pub facts: Option<Vec<serde_json::Value>>,
    pub traited_facts: Option<Vec<serde_json::Value>>,
    pub specialization: Option<SpecializationId>,
    pub icon: String,
}

impl Endpoint for Trait {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/traits";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for Trait {
    type IdType = TraitId;
}
impl BulkEndpoint for Trait {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
