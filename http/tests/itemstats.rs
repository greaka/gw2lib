#![cfg(feature = "blocking")]

use gw2lib::{model::items::itemstats::ItemStat, Requester};

pub mod setup;

#[test]
fn all() {
    let client = crate::setup::setup();
    let _: Vec<ItemStat> = client.all().unwrap();
}
