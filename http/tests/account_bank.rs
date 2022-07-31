#![cfg(feature = "blocking")]

use gw2lib::{model::authenticated::account::bank::Bank, Requester};

pub mod setup;

#[test]
fn eff_testing() {
    let client = setup::setup();
    let _: Bank = client.get().unwrap();
}
