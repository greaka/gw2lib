#![cfg(feature = "blocking")]

use gw2lib::{
    model::{
        items::{finishers::Finisher, materials::MaterialCategory},
        misc::{dungeons::Dungeon, files::GameFile, minis::Mini, quaggans::Quaggan, titles::Title},
    },
    Requester,
};

pub mod setup;

#[test]
fn dungeons() {
    let client = setup::setup();
    let ids: Vec<String> = client.ids::<Dungeon, _>().unwrap();
    assert!(!ids.is_empty());
}

#[test]
fn files() {
    let client = setup::setup();
    let _: GameFile = client.single("map_complete".to_string()).unwrap();
}

#[test]
fn quaggans() {
    let client = setup::setup();
    let _: Quaggan = client.single("404".to_string()).unwrap();
}

#[test]
fn minis() {
    let client = setup::setup();
    let _: Mini = client.single(1u64).unwrap();
}

#[test]
fn titles() {
    let client = setup::setup();
    let _: Title = client.single(1u16).unwrap();
}

#[test]
fn finishers() {
    let client = setup::setup();
    let _: Finisher = client.single(1u32).unwrap();
}

#[test]
fn materials() {
    let client = setup::setup();
    let _: MaterialCategory = client.single(5u32).unwrap();
}
