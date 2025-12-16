mod floors;

use std::collections::BTreeSet;

use serde::{ser::SerializeSeq, Deserialize, Deserializer, Serialize, Serializer};

pub use crate::maps::continents::floors::*;
use crate::{BulkEndpoint, Endpoint, EndpointWithId};

pub type ContinentId = u32;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
#[serde(from = "[u32; 2]", into = "[u32; 2]")]
pub struct Dimensions {
    pub width: u32,
    pub height: u32,
}
impl From<[u32; 2]> for Dimensions {
    fn from([width, height]: [u32; 2]) -> Self {
        Self { width, height }
    }
}
impl From<Dimensions> for [u32; 2] {
    fn from(v: Dimensions) -> Self {
        [v.width, v.height]
    }
}

#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Continent {
    /// The id of the continent.
    pub id: ContinentId,
    /// The name of the continent.
    pub name: String,
    /// The dimensions of the continent.
    pub continent_dims: Dimensions,
    /// The minimal zoom level for use with the map tile service.
    pub min_zoom: u8,
    /// The maximum zoom level for use with the map tile service.
    pub max_zoom: u8,
    /// A list of floors ids available for this continent.
    #[serde(serialize_with = "serialize_floor")]
    pub floors: BTreeSet<ContinentFloorId>,
}

fn serialize_floor<S>(floors: &BTreeSet<ContinentFloorId>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = s.serialize_seq(Some(floors.len()))?;
    for id in floors {
        seq.serialize_element(&id.floor)?;
    }
    seq.end()
}

impl<'de> Deserialize<'de> for Continent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Intermediate {
            id: ContinentId,
            name: String,
            continent_dims: Dimensions,
            min_zoom: u8,
            max_zoom: u8,
            floors: BTreeSet<FloorId>,
        }

        let intermediate: Intermediate = Deserialize::deserialize(deserializer)?;
        let floors = intermediate
            .floors
            .iter()
            .map(|id| ContinentFloorId {
                continent: intermediate.id,
                floor: *id,
            })
            .collect();

        Ok(Continent {
            id: intermediate.id,
            name: intermediate.name,
            continent_dims: intermediate.continent_dims,
            min_zoom: intermediate.min_zoom,
            max_zoom: intermediate.max_zoom,
            floors,
        })
    }
}

impl EndpointWithId for Continent {
    type IdType = ContinentId;
}
impl Endpoint for Continent {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/continents";
    const VERSION: &'static str = "2023-03-31T00:00:00.000Z";
}

impl BulkEndpoint for Continent {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
