#![cfg(feature = "blocking")]

use gw2lib::{
    Requester,
    model::misc::raids::{Raid, RaidId},
};

pub mod setup;

#[test]
fn all() {
    let client = setup::setup();
    let _: Vec<Raid> = client.all().unwrap();
    let _: Raid = client.try_single("forsaken_thicket".into()).unwrap();
}

#[test]
fn ids() {
    let client = setup::setup();
    let _: Vec<RaidId> = client.ids::<Raid>().unwrap();
    let _: Vec<RaidId> = client.try_ids::<Raid>().unwrap();
}

#[test]
fn forsaken_thicket_wings() {
    let client = setup::setup();
    let _: Raid = client.single("forsaken_thicket".into()).unwrap();
    let _: Raid = client.try_single("forsaken_thicket".into()).unwrap();
}

#[test]
fn bastion_of_the_penitent_wings() {
    let client = setup::setup();
    let _: Raid = client.single("bastion_of_the_penitent".into()).unwrap();
    let _: Raid = client.try_single("bastion_of_the_penitent".into()).unwrap();
}

#[test]
fn hall_of_chains_wings() {
    let client = setup::setup();
    let _: Raid = client.single("hall_of_chains".into()).unwrap();
    let _: Raid = client.try_single("hall_of_chains".into()).unwrap();
}

#[test]
fn the_key_of_ahdashim_wings() {
    let client = setup::setup();
    let _: Raid = client.single("the_key_of_ahdashim".into()).unwrap();
    let _: Raid = client.try_single("the_key_of_ahdashim".into()).unwrap();
}
