#![cfg(feature = "blocking")]

use gw2lib::{
    model::achievements::{
        categories::AchievementCategory, groups::AchievementGroup, Achievement,
    },
    Requester,
};

pub mod setup;

#[test]
fn achievement_single() {
    let client = setup::setup();
    let _: Achievement = client.single(739).unwrap();
}

#[test]
fn achievements() {
    let client = setup::setup();
    let _: Vec<Achievement> = client.all().unwrap();
}

#[test]
fn achievement_categories() {
    let client = setup::setup();
    let _: Vec<AchievementCategory> = client.all().unwrap();
}

#[test]
fn achievement_groups() {
    let client = setup::setup();
    let _: Vec<AchievementGroup> = client.all().unwrap();
}
