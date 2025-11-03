#![cfg(feature = "blocking")]

use gw2lib::{model::achievements::{
    Achievements,
    categories::AchievementCategories,
    groups::AchievementGroups,
}, Requester};

pub mod setup;

#[test]
fn achievements() {
    let client = setup::setup();
    let _: Achievements = client.get().unwrap();
}

#[test]
fn achievement_categories() {
    let client = setup::setup();
    let _: AchievementCategories = client.get().unwrap();
}

#[test]
fn achievement_groups() {
    let client = setup::setup();
    let _: AchievementGroups = client.get().unwrap();
}
