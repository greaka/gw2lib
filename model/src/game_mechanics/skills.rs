use serde::{Deserialize, Serialize};

use crate::game_mechanics::traits::TraitId;
pub use crate::{authenticated::characters::Profession, items::WeaponType};

pub type SkillId = u32;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum SkillType {
    Bundle,
    Elite,
    Heal,
    Profession,
    Utility,
    Weapon,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Fact {
    pub text:  String,
    pub icon:  Option<String>,
    pub _type: FactsType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TraitedFact {
    #[serde(flatten)]
    pub fact:           Fact,
    pub requires_trait: TraitId,
    /// array index of Fact
    pub overrides:      Option<u8>,
}
