pub type SpecializationId = u16;

use serde::{Deserialize, Serialize};

use crate::{game_mechanics::traits::TraitId, BulkEndpoint, Endpoint, EndpointWithId};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Specialization {
    pub id: SpecializationId,
    pub name: String,
    pub profession: String,
    pub elite: bool,
    pub minor_traits: Vec<TraitId>,
    pub major_traits: Vec<TraitId>,
    pub icon: String,
    pub background: String,
    pub weapon_trait: Option<TraitId>,
    pub profession_icon: Option<String>,
    pub profession_icon_big: Option<String>,
}

impl Endpoint for Specialization {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/specializations";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for Specialization {
    type IdType = SpecializationId;
}
impl BulkEndpoint for Specialization {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
