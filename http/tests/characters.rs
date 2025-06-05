#![cfg(feature = "blocking")]

use gw2lib::{
    model::authenticated::characters::{
        Backstory, Character, CharacterId, Core, Crafting, Equipment, Inventory, Recipes, Training,
    },
    Requester,
};

use crate::setup::character_name;

pub mod setup;

#[test]
fn elementalist() {
    let client = setup::setup();
    let _: Character = client.single(character_name()).unwrap();
    let _: Character = client.try_single(character_name()).unwrap();
}

#[test]
fn all_chars() {
    let client = setup::setup();
    let _: Vec<Character> = client.all().unwrap();
    let _: Character = client.try_single(character_name()).unwrap();
}

#[test]
fn ids() {
    let client = setup::setup();
    let _: Vec<CharacterId> = client.ids::<Character>().unwrap();
    let _: Vec<CharacterId> = client.try_ids::<Character>().unwrap();
}

#[test]
fn core() {
    let client = setup::setup();
    let _: Core = client.single(character_name()).unwrap();
    let _: Core = client.try_single(character_name()).unwrap();
}

#[test]
fn backstory() {
    let client = setup::setup();
    let _: Backstory = client.single(character_name()).unwrap();
    let _: Backstory = client.try_single(character_name()).unwrap();
}

#[test]
fn crafting() {
    let client = setup::setup();
    let _: Crafting = client.single(character_name()).unwrap();
    let _: Crafting = client.try_single(character_name()).unwrap();
}

#[test]
fn equipment() {
    let client = setup::setup();
    let _: Equipment = client.single(character_name()).unwrap();
    let _: Equipment = client.try_single(character_name()).unwrap();
}

#[test]
fn inventory() {
    let client = setup::setup();
    let _: Inventory = client.single(character_name()).unwrap();
    let _: Inventory = client.try_single(character_name()).unwrap();
}

#[test]
fn recipes() {
    let client = setup::setup();
    let _: Recipes = client.single(character_name()).unwrap();
    let _: Recipes = client.try_single(character_name()).unwrap();
}

#[test]
fn training() {
    let client = setup::setup();
    let _: Training = client.single(character_name()).unwrap();
    let _: Training = client.try_single(character_name()).unwrap();
}
