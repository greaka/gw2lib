#![cfg(feature = "blocking")]

use gw2lib::{
    model::daily_rewards::{
        dailycrafting::DailyCrafting, mapchests::MapChest, worldbosses::WorldBoss,
    },
    Requester,
};

pub mod setup;

#[test]
fn dailycrafting() {
    let client = setup::setup();
    let ids: Vec<String> = client.ids::<DailyCrafting, _>().unwrap();
    assert!(!ids.is_empty());
}

#[test]
fn mapchests() {
    let client = setup::setup();
    let ids: Vec<String> = client.ids::<MapChest, _>().unwrap();
    assert!(!ids.is_empty());
}

#[test]
fn worldbosses() {
    let client = setup::setup();
    let ids: Vec<String> = client.ids::<WorldBoss, _>().unwrap();
    assert!(!ids.is_empty());
}
