#![cfg(feature = "blocking")]

use gw2api::{
    model::authenticated::characters::{
        Backstory, Character, CharacterId, Core, Crafting, Equipment, Inventory, Recipes, Training,
    },
    Requester,
};

pub mod setup;

#[test]
fn elementalist() {
    let client = setup::setup();
    let _: Character = client.single("Eff Testing Ele".to_string()).unwrap();
}

#[test]
fn all_chars() {
    let client = setup::setup();
    let _: Vec<Character> = client.all().unwrap();
}

#[test]
fn ids() {
    let client = setup::setup();
    let _: Vec<CharacterId> = client.ids::<Character, _>().unwrap();
}

#[test]
fn core() {
    let client = setup::setup();
    let _: Core = client.single("Eff Testing Ele".to_string()).unwrap();
}

#[test]
fn backstory() {
    let client = setup::setup();
    let _: Backstory = client.single("Eff Testing Ele".to_string()).unwrap();
}

#[test]
fn crafting() {
    let client = setup::setup();
    let _: Crafting = client.single("Eff Testing Ele".to_string()).unwrap();
}

#[test]
fn equipment() {
    let client = setup::setup();
    let _: Equipment = client.single("Eff Testing Ele".to_string()).unwrap();
}

#[test]
fn inventory() {
    let client = setup::setup();
    let _: Inventory = client.single("Eff Testing Ele".to_string()).unwrap();
}

#[test]
fn recipes() {
    let client = setup::setup();
    let _: Recipes = client.single("Eff Testing Ele".to_string()).unwrap();
}

#[test]
fn training() {
    let client = setup::setup();
    let _: Training = client.single("Eff Testing Ele".to_string()).unwrap();
}
