#![cfg(feature = "blocking")]

use gw2lib::{
	model::game_mechanics::pets::{Pet, PetId},
	Requester
};

pub mod setup;

#[test]
fn all() {
    let client = setup::setup();
    assert!(client.all::<Pet, PetId>().is_ok());
}
