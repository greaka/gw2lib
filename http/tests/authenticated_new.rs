#![cfg(feature = "blocking")]

use gw2lib::{
    model::authenticated::{
        characters::Character,
        commerce::{CurrentBuyTransactions, HistoryBuyTransactions},
        pvp::{PvpStandings, PvpStats},
    },
    Requester,
};

use crate::setup::character_name;

pub mod setup;

#[test]
fn heropoints() {
    let client = setup::setup();
    use gw2lib::model::authenticated::characters::Heropoints;
    let _: Heropoints = client.single(character_name()).unwrap();
}

#[test]
fn sab() {
    let client = setup::setup();
    use gw2lib::model::authenticated::characters::Sab;
    let char_ids: Vec<gw2lib::model::authenticated::characters::CharacterId> =
        client.ids::<Character, _>().unwrap();
    if let Some(name) = char_ids.first() {
        let _: Sab = client.single(name.clone()).unwrap();
    }
}

#[test]
fn pvp_stats() {
    let client = setup::setup();
    let _: PvpStats = client.get().unwrap();
}

#[test]
fn pvp_standings() {
    let client = setup::setup();
    let _: PvpStandings = client.get().unwrap();
}

#[test]
fn commerce_current_buys() {
    let client = setup::setup();
    let _: CurrentBuyTransactions = client.get().unwrap();
}

#[test]
fn commerce_history_buys() {
    let client = setup::setup();
    let _: HistoryBuyTransactions = client.get().unwrap();
}
