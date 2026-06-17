#![cfg(feature = "blocking")]

use gw2lib::{
    model::game_mechanics::{
        legends::Legend,
        masteries::Mastery,
        mounts::{MountSkin, MountType},
        outfits::Outfit,
        pets::Pet,
        professions::ProfessionInfo,
        races::Race,
        skills::Skill,
        specializations::Specialization,
        traits::Trait,
    },
    Requester,
};

pub mod setup;

#[test]
fn masteries() {
    let client = setup::setup();
    let ids: Vec<u32> = client.ids::<Mastery, _>().unwrap();
    assert!(!ids.is_empty());
    let _: Mastery = client.single(ids[0]).unwrap();
}

#[test]
fn mount_skins() {
    let client = setup::setup();
    let _: MountSkin = client.single(1u32).unwrap();
}

#[test]
fn mount_types() {
    let client = setup::setup();
    let ids: Vec<String> = client.ids::<MountType, _>().unwrap();
    assert!(!ids.is_empty());
}

#[test]
fn outfits() {
    let client = setup::setup();
    let _: Outfit = client.single(1u32).unwrap();
}

#[test]
fn pets() {
    let client = setup::setup();
    let _: Pet = client.single(1u16).unwrap();
}

#[test]
fn professions() {
    let client = setup::setup();
    let ids: Vec<String> = client.ids::<ProfessionInfo, _>().unwrap();
    assert!(!ids.is_empty());
    let _: ProfessionInfo = client.single(ids[0].clone()).unwrap();
}

#[test]
fn races() {
    let client = setup::setup();
    let ids: Vec<String> = client.ids::<Race, _>().unwrap();
    assert!(!ids.is_empty());
}

#[test]
fn skills() {
    let client = setup::setup();
    let _: Skill = client.single(1110u32).unwrap();
}

#[test]
fn specializations() {
    let client = setup::setup();
    let _: Specialization = client.single(1u16).unwrap();
}

#[test]
fn traits() {
    let client = setup::setup();
    let _: Trait = client.single(214u16).unwrap();
}

#[test]
fn legends() {
    let client = setup::setup();
    let ids: Vec<String> = client.ids::<Legend, _>().unwrap();
    assert!(!ids.is_empty());
    let _: Legend = client.single(ids[0].clone()).unwrap();
}
