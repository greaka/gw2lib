#![cfg(feature = "blocking")]

use gw2lib::{model::items::itemstats::ItemStat, Requester};

pub mod setup;

#[test]
fn all() {
    let client = setup::setup();
    let _: Vec<ItemStat> = client.all().unwrap();
    let _: ItemStat = client.try_single(161).unwrap();
}
