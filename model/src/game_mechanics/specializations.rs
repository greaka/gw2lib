use serde::{Deserialize, Serialize};

use crate::game_mechanics::traits::TraitId;
use crate::{BulkEndpoint, Endpoint, EndpointWithId};

pub type SpecializationId = u16;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Specialization {
    pub id: SpecializationId,
    pub name: String,
    pub profession: String,
    pub elite: bool,
    pub icon: String,
    pub background: String,
    pub minor_traits: Vec<TraitId>,
    pub major_traits: Vec<TraitId>,
    pub profession_icon: Option<String>,
    pub profession_icon_big: Option<String>,
    pub weapon_trait: Option<TraitId>,
}

impl Endpoint for Specialization {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/specializations";
    const VERSION: &'static str = "2026-06-29T00:00:00.000Z";
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
