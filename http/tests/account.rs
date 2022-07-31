#![cfg(feature = "blocking")]

use gw2lib::{model::authenticated::account::Account, Requester};

pub mod setup;

#[test]
fn eff_testing() {
    let client = setup::setup();
    let _: Account = client.get().unwrap();
}
