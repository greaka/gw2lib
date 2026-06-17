#![cfg(feature = "blocking")]

use gw2lib::{
    model::story::{
        backstory::{BackstoryAnswer, BackstoryQuestion},
        stories::{Story, StorySeason},
    },
    Requester,
};

pub mod setup;

#[test]
fn backstory_answers() {
    let client = setup::setup();
    let ids: Vec<String> = client.ids::<BackstoryAnswer, _>().unwrap();
    assert!(!ids.is_empty());
}

#[test]
fn backstory_questions() {
    let client = setup::setup();
    let _: BackstoryQuestion = client.single(7u32).unwrap();
}

#[test]
fn stories() {
    let client = setup::setup();
    let ids: Vec<u32> = client.ids::<Story, _>().unwrap();
    assert!(!ids.is_empty());
}

#[test]
fn story_seasons() {
    let client = setup::setup();
    let ids: Vec<String> = client.ids::<StorySeason, _>().unwrap();
    assert!(!ids.is_empty());
}
