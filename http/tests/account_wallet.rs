#![cfg(feature = "blocking")]

use gw2lib::Requester;
use gw2lib_model::authenticated::account::wallet::Wallet;

pub mod setup;

#[test]
fn eff_testing() {
    let client = setup::setup();
    let _: Wallet = client.get().unwrap();
}
