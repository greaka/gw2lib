use serde::{Deserialize, Serialize};
use crate::maps::continents::ContinentId;
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct FloorId {
    pub continent: ContinentId,
    pub floor: i8,
}
