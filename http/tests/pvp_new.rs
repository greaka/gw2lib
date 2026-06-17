#![cfg(feature = "blocking")]

use gw2lib::{
    model::pvp::{amulets::Amulet, ranks::PvpRank, seasons::PvpSeason},
    Requester,
};

pub mod setup;

#[test]
fn pvp_amulets() {
    let client = setup::setup();
    let _: Amulet = client.single(4u16).unwrap();
}

#[test]
fn pvp_ranks() {
    let client = setup::setup();
    let _: PvpRank = client.single(1u32).unwrap();
}

#[test]
fn pvp_seasons() {
    let client = setup::setup();
    let ids: Vec<String> = client.ids::<PvpSeason, _>().unwrap();
    assert!(!ids.is_empty());
}
