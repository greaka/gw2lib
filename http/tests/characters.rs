#![cfg(feature = "blocking")]

use gw2lib::{
    Requester,
    model::authenticated::characters::{
        Backstory, Character, CharacterId, Core, Crafting, Equipment, Inventory, Recipes, Training,
    },
};

use crate::setup::character_name;

pub mod setup;

#[test]
fn elementalist() {
    let client = setup::setup();
    let _: Character = client.single(character_name()).unwrap();
}

#[test]
fn all_chars() {
    let client = setup::setup();
    let _: Vec<Character> = client.all().unwrap();
}

#[test]
fn ids() {
    let client = setup::setup();
    let _: Vec<CharacterId> = client.ids::<Character>().unwrap();
}

#[test]
fn core() {
    let client = setup::setup();
    let _: Core = client.single(character_name()).unwrap();
}

#[test]
fn backstory() {
    let client = setup::setup();
    let _: Backstory = client.single(character_name()).unwrap();
}

#[test]
fn crafting() {
    let client = setup::setup();
    let _: Crafting = client.single(character_name()).unwrap();
}

#[test]
fn equipment() {
    let client = setup::setup();
    let _: Equipment = client.single(character_name()).unwrap();
}

#[test]
fn inventory() {
    let client = setup::setup();
    let _: Inventory = client.single(character_name()).unwrap();
}

#[test]
fn recipes() {
    let client = setup::setup();
    let _: Recipes = client.single(character_name()).unwrap();
}

#[test]
fn training() {
    let client = setup::setup();
    let _: Training = client.single(character_name()).unwrap();
}
