#![cfg(feature = "blocking")]

use gw2lib::{model::authenticated::account::wallet::Wallet, Requester};

pub mod setup;

#[test]
fn eff_testing() {
    let client = setup::setup();
    let _: Wallet = client.get().unwrap();
}
