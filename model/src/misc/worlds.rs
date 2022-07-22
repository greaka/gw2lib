use serde::{Deserialize, Serialize};

use crate::*;

pub type WorldId = u16;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum PopulationLevel {
    Medium,
    High,
    VeryHigh,
    Full,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct World {
    id: WorldId,
    name: String,
    population: PopulationLevel,
}

impl Endpoint for World {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/worlds";
    const VERSION: &'static str = "2022-07-22T00:00:00.000Z";
}

impl EndpointWithId for World {
    type IdType = WorldId;
}

impl BulkEndpoint for World {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
