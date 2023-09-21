#![cfg(feature = "blocking")]

use gw2lib::{model::authenticated::account::inventory::AccountInventory, Requester};

pub mod setup;

#[test]
fn eff_testing() {
    let client = setup::setup();
    let _: AccountInventory = client.get().unwrap();
}
