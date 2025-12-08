use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

use serde::{Deserialize, Serialize};

use crate::{
    maps::{
        continents::{ContinentId, Dimensions},
        MapId,
    },
    Endpoint, EndpointWithId,
};

pub type FloorId = i16;
pub type RegionId = u8;
pub type MasteryPointId = u16;
pub type PointOfInterestId = u16;
pub type GodShrineId = u8;
pub type TaskId = u16;
pub type SectorId = u16;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ContinentFloorId {
    pub continent: ContinentId,
    pub floor: FloorId,
}

impl From<(ContinentId, FloorId)> for ContinentFloorId {
    fn from(value: (ContinentId, FloorId)) -> Self {
        Self {
            continent: value.0,
            floor: value.1,
        }
    }
}

impl Display for ContinentFloorId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/floors/{}", self.continent, self.floor)
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
#[serde(from = "[f32; 2]", into = "[f32; 2]")]
pub struct Coordinates {
    pub x: f32,
    pub y: f32,
}
impl From<[f32; 2]> for Coordinates {
    fn from([x, y]: [f32; 2]) -> Self {
        Self { x, y }
    }
}
impl From<Coordinates> for [f32; 2] {
    fn from(v: Coordinates) -> Self {
        [v.x, v.y]
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
#[serde(from = "[Coordinates; 2]", into = "[Coordinates; 2]")]
pub struct ContinentRectangle {
    pub top_left: Coordinates,
    pub bottom_right: Coordinates,
}
impl From<[Coordinates; 2]> for ContinentRectangle {
    fn from([top_left, bottom_right]: [Coordinates; 2]) -> Self {
        Self {
            top_left,
            bottom_right,
        }
    }
}
impl From<ContinentRectangle> for [Coordinates; 2] {
    fn from(v: ContinentRectangle) -> Self {
        [v.top_left, v.bottom_right]
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
#[serde(from = "[Coordinates; 2]", into = "[Coordinates; 2]")]
pub struct MapRectangle {
    pub bottom_left: Coordinates,
    pub top_right: Coordinates,
}
impl From<[Coordinates; 2]> for MapRectangle {
    fn from([bottom_left, top_right]: [Coordinates; 2]) -> Self {
        Self {
            bottom_left,
            top_right,
        }
    }
}
impl From<MapRectangle> for [Coordinates; 2] {
    fn from(v: MapRectangle) -> Self {
        [v.bottom_left, v.top_right]
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub enum PointOfInterestType {
    Landmark,
    Waypoint,
    Vista,
    Unlock,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct PointOfInterest {
    /// The POI id.
    pub id: PointOfInterestId,
    /// The POI name, if any.
    pub name: Option<String>,
    /// The POI type.
    #[serde(rename = "type")]
    pub _type: PointOfInterestType,
    /// The floor of this POI.
    pub floor: FloorId,
    /// The POI coordinates.
    pub coord: Coordinates,
    /// The POI chat link.
    pub chat_link: String,
    /// For [`Unlock`](PointOfInterestType::Unlock) type, provides the render
    /// service url for the POI's icon.
    pub icon: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct GodShrine {
    /// The shrine id.
    pub id: GodShrineId,
    /// The shrine name when not contested.
    pub name: String,
    /// The shrine name when contested.
    pub name_contested: String,
    /// The shrine coordinates.
    pub coord: Coordinates,
    /// The associated waypoint id.
    pub poi_id: PointOfInterestId,
    /// The render service url for the shrine's icon when not contested.
    pub icon: String,
    /// The render service url for the shrine's icon when contested.
    pub icon_contested: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Task {
    /// The task id.
    pub id: TaskId,
    /// The objective or name of the task.
    pub objective: String,
    /// The level of the task.
    pub level: u8,
    /// The coordinates of the task.
    pub coord: Coordinates,
    /// A list of coordinates marking the boundary of the task.
    pub bounds: Vec<Coordinates>,
    /// The task chat link (provides an invalid link if attempting to display
    /// in-game).
    pub chat_link: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct SkillChallenge {
    /// The hero challenge id, if any.
    ///
    /// It is formed of two numbers separated by a dash. The first number
    /// represents the expansion (0 for Core Tyria, 1 for Heart of Thorns
    /// and 2 for Path of Fire), and therefore could be used to change the
    /// hero challenge map marker icon. If the first number and dash prefix is
    /// removed from the string, the second number is no longer unique among
    /// other hero challenges.
    pub id: Option<String>,
    /// The coordinates of this hero challenge.
    pub coord: Coordinates,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Sector {
    /// The sector id.
    pub id: SectorId,
    /// The name of the sector, if any.
    pub name: Option<String>,
    /// The level of the sector.
    pub level: u8,
    /// The coordinates of the sector (this is usually the center position).
    pub coord: Coordinates,
    /// A list of coordinates marking the boundary of the sector.
    pub bounds: Vec<Coordinates>,
    /// The sector chat link (provides an invalid link if attempting to display
    /// in-game).
    pub chat_link: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Adventure {
    /// The adventure UUID.
    pub id: String,
    /// The name of the adventure.
    pub name: String,
    /// The description of the adventure.
    pub description: String,
    /// The coordinates of the start of the adventure.
    pub coord: Coordinates,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub enum MasteryPointRegion {
    /// Core Tyria, in red.
    Tyria,
    /// Heart of Thorns, in green.
    Maguuma,
    /// Path of Fire, in purple
    Desert,
    /// Living World Season 5, in blue.
    Tundra,
    /// End of Dragons
    Jade,
    /// Secrets of the Obscure
    Sky,
    /// Janthir Wilds
    #[serde(alias = "Unknown")]
    Janthir,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct MasteryPoint {
    /// The mastery point id.
    pub id: MasteryPointId,
    /// The type of mastery.
    pub region: MasteryPointRegion,
    /// The coordinates of the start of the adventure.
    pub coord: Coordinates,
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
    /// The coordinates of the map label.
    pub label_coord: Option<Coordinates>,
    /// The dimensions of the map.
    pub map_rect: MapRectangle,
    /// The dimensions of the map within the continent coordinate system.
    pub continent_rect: ContinentRectangle,
    /// The list of points of interest (landmarks, waypoints, vistas, etc) of
    /// the map.
    pub points_of_interest: HashMap<PointOfInterestId, PointOfInterest>,
    pub god_shrines: Option<Vec<GodShrine>>,
    /// The list of renown hearts of the map.
    pub tasks: HashMap<TaskId, Task>,
    /// The list of hero challenges of the map.
    pub skill_challenges: Vec<SkillChallenge>,
    /// The list of areas of the map.
    pub sectors: HashMap<SectorId, Sector>,
    /// The list of adventures of the map.
    pub adventures: Vec<Adventure>,
    /// The list of mastery points of the map.
    pub mastery_points: Vec<MasteryPoint>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Region {
    /// The region id.
    pub id: RegionId,
    /// The region name.
    pub name: String,
    ///  The coordinates of the region label.
    pub label_coord: Coordinates,
    /// The dimensions of the region in the continent.
    pub continent_rect: ContinentRectangle,
    /// The list of maps in this region.
    pub maps: HashMap<MapId, Map>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Floor {
    pub id: FloorId,
    /// The dimensions of the texture.
    pub texture_dims: Dimensions,
    /// If present, it represents a rectangle of downloadable textures. Every
    /// tile coordinate outside this rectangle is not available on the tile
    /// server.
    pub clamped_view: Option<ContinentRectangle>,
    /// The list of regions in on this floor.
    pub regions: HashMap<RegionId, Region>,
}

impl EndpointWithId for Floor {
    type IdType = ContinentFloorId;

    fn format_id(id: &Self::IdType) -> String {
        id.to_string()
    }
}

impl Endpoint for Floor {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/continents";
    const VERSION: &'static str = "2023-03-31T00:00:00.000Z";
}
