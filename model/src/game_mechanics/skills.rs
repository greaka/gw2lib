use serde::{Deserialize, Serialize};

use crate::game_mechanics::traits::TraitId;
pub use crate::{authenticated::characters::Profession, items::WeaponType};

pub type SkillId = u32;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SkillType {
    Bundle,
    Elite,
    Heal,
    Profession,
    Utility,
    Weapon,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Slot {
    Downed_1,
    Downed_2,
    Downed_3,
    Downed_4,
    Pet,
    Profession_1,
    Profession_2,
    Profession_3,
    Profession_4,
    Profession_5,
    Utility,
    Weapon_1,
    Weapon_2,
    Weapon_3,
    Weapon_4,
    Weapon_5,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum FactsType {
    AttributeAdjust,
    Buff,
    ComboField,
    ComboFinisher,
    Damage,
    Distance,
    Duration,
    Heal,
    HealingAdjust,
    NoData,
    Number,
    Percent,
    PrefixedBuff,
    Radius,
    Range,
    Recharge,
    Time,
    Unblockable,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Fact {
    pub text: String,
    pub icon: Option<String>,
    pub _type: FactsType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TraitedFact {
    #[serde(flatten)]
    pub fact: Fact,
    pub requires_trait: TraitId,
    /// array index of Fact
    pub overrides: Option<u8>,
}

use crate::{BulkEndpoint, Endpoint, EndpointWithId};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Skill {
    pub id: SkillId,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub chat_link: Option<String>,
    #[serde(rename = "type")]
    pub _type: Option<String>,
    pub weapon_type: Option<String>,
    pub professions: Option<Vec<String>>,
    pub slot: Option<String>,
    pub facts: Option<Vec<serde_json::Value>>,
    pub traited_facts: Option<Vec<serde_json::Value>>,
    pub categories: Option<Vec<String>>,
    pub attunement: Option<String>,
    pub cost: Option<u32>,
    pub dual_wield: Option<String>,
    pub flip_skill: Option<SkillId>,
    pub initiative: Option<u8>,
    pub next_chain: Option<SkillId>,
    pub prev_chain: Option<SkillId>,
    pub transform_skills: Option<Vec<SkillId>>,
    pub bundle_skills: Option<Vec<SkillId>>,
    pub toolbelt_skill: Option<SkillId>,
    pub specialization: Option<crate::game_mechanics::specializations::SpecializationId>,
    pub subtype: Option<String>,
    pub flags: Option<Vec<String>>,
    pub log: Option<String>,
    pub ranges: Option<Vec<u32>>,
    pub recharge: Option<f32>,
}

impl Endpoint for Skill {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/skills";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for Skill {
    type IdType = SkillId;
}
impl BulkEndpoint for Skill {
    const ALL: bool = false;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
