#![cfg(feature = "blocking")]

use gw2lib::{
    model::wvw::{
        abilities::Ability, matches::WvwMatch, objectives::WvwObjective, ranks::WvwRank,
        upgrades::WvwUpgrade,
    },
    Requester,
};

pub mod setup;

#[test]
fn abilities() {
    let client = setup::setup();
    let _: Ability = client.single(2u32).unwrap();
}

#[test]
fn matches() {
    let client = setup::setup();
    let ids: Vec<String> = client.ids::<WvwMatch, _>().unwrap();
    assert!(!ids.is_empty());
    let _: WvwMatch = client.single(ids[0].clone()).unwrap();
}

#[test]
fn objectives() {
    let client = setup::setup();
    let _: WvwObjective = client.single("1099-99".to_string()).unwrap();
}

#[test]
fn ranks() {
    let client = setup::setup();
    let _: WvwRank = client.single(1u32).unwrap();
}

#[test]
fn upgrades() {
    let client = setup::setup();
    let _: WvwUpgrade = client.single(5u32).unwrap();
}
