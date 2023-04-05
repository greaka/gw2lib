use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::{
    maps::continents::{ContinentId, ContinentRectangle, FloorId, MapRectangle, RegionId},
    BulkEndpoint, Endpoint, EndpointWithId,
};

pub mod continents;

pub type MapId = u32;

#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub enum MapType {
    /// The blue home borderlands in WvW.
    BlueHome,
    /// The center map in WvW (Eternal Battlegrounds).
    Center,
    /// The Edge of the Mists map in WvW.
    EdgeOfTheMists,
    /// The green home borderlands in WvW.
    GreenHome,
    /// An instanced map.
    Instance,
    /// At present only a WvW map that houses a jumping puzzle (Obsidian
    /// Sanctum).
    JumpPuzzle,
    /// Open world map.
    Public,
    /// PvP or activity map.
    Pvp,
    /// The red home borderlands in WvW.
    RedHome,
    /// The tutorial missions for newly created characters.
    Tutorial,
    Unknown,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Map {
    /// The map id.
    pub id: MapId,
    /// The map name.
    pub name: String,
    /// The minimum level on this map.
    pub min_level: u8,
    /// The maximum level on this map.
    pub max_level: u8,
    /// The default floor of the map.
    pub default_floor: FloorId,
    /// The map type.
    #[serde(rename = "type")]
    pub _type: MapType,
    /// The list of available floors for the map.
    pub floors: BTreeSet<FloorId>,
    /// The id of the region this map belongs to, if any.
    pub region_id: Option<RegionId>,
    /// The name of the region this map belongs to, if any.
    pub region_name: Option<String>,
    /// The id of the continent this map belongs to, if any.
    pub continent_id: Option<ContinentId>,
    /// The name of the continent this map belongs to, if any.
    pub continent_name: Option<String>,
    /// The dimensions of the map.
    pub map_rect: MapRectangle,
    /// The dimensions of the map within the continent coordinate system.
    pub continent_rect: ContinentRectangle,
}

impl EndpointWithId for Map {
    type IdType = MapId;
}
impl Endpoint for Map {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/maps";
    const VERSION: &'static str = "2023-04-02T00:00:00.000Z";
}

impl BulkEndpoint for Map {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
