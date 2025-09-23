#![cfg(feature = "blocking")]

use gw2lib::{
    Requester,
    model::game_mechanics::pets::{Pet, PetId},
};

pub mod setup;

#[test]
fn all() {
    let client = setup::setup();
    let _: Vec<Pet> = client.all().unwrap();
}

#[test]
fn ids() {
    let client = setup::setup();
    let _: Vec<PetId> = client.ids::<Pet>().unwrap();
}
