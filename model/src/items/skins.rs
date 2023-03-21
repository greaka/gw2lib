use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::{
    items::{
        ArmorSlot, DamageType, GatheringToolsType, Rarity, Restrictions, WeaponType, WeightClass,
    },
    misc::colors::ColorId,
    BulkEndpoint, Endpoint, EndpointWithId,
};

pub type SkinId = u32;

#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub enum SkinType {
    Armor,
    Back,
    Gathering,
    Weapon,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub enum Flags {
    /// Displayed in the account wardrobe.
    ShowInWardrobe,
    /// Applying the skin is free.
    NoCost,
    /// The skin is hidden until it is unlocked.
    HideIfLocked,
    /// The skin overrides item rarity when applied.
    OverrideRarity,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub enum Material {
    Cloth,
    Leather,
    Metal,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct DyeSlot {
    pub color_id: ColorId,
    pub material: Material,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub enum RaceGender {
    AsuraMale,
    AsuraFemale,
    CharrMale,
    CharrFemale,
    HumanMale,
    HumanFemale,
    NornMale,
    NornFemale,
    SylvariMale,
    SylvariFemale,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct DyeSlots {
    pub default: Vec<Option<DyeSlot>>,
    pub overrides: BTreeMap<RaceGender, Vec<Option<DyeSlot>>>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct ArmorDetails {
    #[serde(rename = "type")]
    pub _type: ArmorSlot,
    pub weight_class: WeightClass,
    pub dye_slots: Option<DyeSlots>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct GatheringToolsDetails {
    #[serde(rename = "type")]
    pub _type: GatheringToolsType,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct WeaponDetails {
    #[serde(rename = "type")]
    pub _type: WeaponType,
    pub damage_type: DamageType,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "details")]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub enum Details {
    Armor(ArmorDetails),
    Back,
    Gathering(GatheringToolsDetails),
    Weapon(WeaponDetails),
}

impl From<Details> for SkinType {
    fn from(d: Details) -> Self {
        match d {
            Details::Armor(_) => SkinType::Armor,
            Details::Back => SkinType::Back,
            Details::Gathering(_) => SkinType::Gathering,
            Details::Weapon(_) => SkinType::Weapon,
        }
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Skin {
    pub id: SkinId,
    pub name: String,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub rarity: Rarity,
    pub flags: BTreeSet<Flags>,
    pub restrictions: BTreeSet<Restrictions>,
    #[serde(flatten)]
    pub details: Details,
}

impl EndpointWithId for Skin {
    type IdType = SkinId;
}

impl Endpoint for Skin {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/skins";
    const VERSION: &'static str = "2023-03-20T19:00:00.000Z";
}

impl BulkEndpoint for Skin {
    const ALL: bool = false;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
