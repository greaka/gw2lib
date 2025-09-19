#![cfg(feature = "blocking")]

use gw2lib::{Requester, model::authenticated::account::Account};
use gw2lib_model::authenticated::account::{
    bank::Bank, inventory::AccountInventory, materials::AccountMaterials, raids::RaidEvent,
    wallet::Wallet,
};

pub mod setup;

#[test]
fn account() {
    let client = setup::setup();
    let _: Account = client.get().unwrap();
    let _: Account = client.try_get().unwrap();
}

#[test]
fn bank() {
    let client = setup::setup();
    let _: Bank = client.get().unwrap();
    let _: Bank = client.try_get().unwrap();
}

#[test]
fn inventory() {
    let client = setup::setup();
    let _: AccountInventory = client.get().unwrap();
    let _: AccountInventory = client.try_get().unwrap();
}

#[test]
fn materials() {
    let client = setup::setup();
    let _: AccountMaterials = client.get().unwrap();
    let _: AccountMaterials = client.try_get().unwrap();
}

#[test]
fn raids() {
    let client = setup::setup();
    let _: RaidEvent = client.get().unwrap();
    let _: RaidEvent = client.try_get().unwrap();
}

#[test]
fn wallet() {
    let client = setup::setup();
    let _: Wallet = client.get().unwrap();
    let _: Wallet = client.try_get().unwrap();
}
