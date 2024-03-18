use serde::{Deserialize, Serialize};

use crate::*;
pub use crate::game_mechanics::skills::SkillId;

pub type PetId = u16;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct PetSkill {
	pub id: SkillId,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Pet {
	pub id: PetId,
	pub name: String,
	pub description: String,
	pub icon: Option<String>,
	pub skills: Vec<PetSkill>,
}

impl Endpoint for Pet {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/pets";
    const VERSION: &'static str = "2024-03-09T00:00:00.000Z";
}

impl EndpointWithId for Pet {
	type IdType = PetId;
}

impl BulkEndpoint for Pet {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
