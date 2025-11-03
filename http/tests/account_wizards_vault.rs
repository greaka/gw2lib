#![cfg(feature = "blocking")]

use gw2lib::{
    model::authenticated::account::wizards_vault::{
        daily::WizardsVaultDailies, listings::WizardsVaultListings, special::WizardsVaultSpecials,
        weekly::WizardsVaultWeeklies,
    },
    Requester,
};

pub mod setup;

#[test]
fn dailies() {
    let client = setup::setup();
    let _: WizardsVaultDailies = client.get().unwrap();
}

#[test]
fn listings() {
    let client = setup::setup();
    let _: WizardsVaultListings = client.get().unwrap();
}

#[test]
fn specials() {
    let client = setup::setup();
    let _: WizardsVaultSpecials = client.get().unwrap();
}

#[test]
fn weeklies() {
    let client = setup::setup();
    let _: WizardsVaultWeeklies = client.get().unwrap();
}
