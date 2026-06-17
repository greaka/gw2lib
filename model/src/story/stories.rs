use serde::{Deserialize, Serialize};

use crate::{BulkEndpoint, Endpoint, EndpointWithId};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct StoryChapter {
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Story {
    pub id: u32,
    pub season: String,
    pub name: String,
    pub description: String,
    pub timeline: String,
    pub level: u8,
    pub races: Option<Vec<String>>,
    pub order: u32,
    pub chapters: Vec<StoryChapter>,
    pub professions: Option<Vec<String>>,
    pub flags: Option<Vec<String>>,
}

impl Endpoint for Story {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/stories";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for Story {
    type IdType = u32;
}
impl BulkEndpoint for Story {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct StorySeason {
    pub id: String,
    pub name: String,
    pub order: u32,
    pub stories: Vec<u32>,
}

impl Endpoint for StorySeason {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/stories/seasons";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for StorySeason {
    type IdType = String;
}
impl BulkEndpoint for StorySeason {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
