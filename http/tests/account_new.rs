#![cfg(feature = "blocking")]

use gw2lib::{
    model::authenticated::account::{
        achievements::AccountAchievements,
        dailycrafting::AccountDailyCrafting,
        dungeons::AccountDungeons,
        dyes::AccountDyes,
        finishers::AccountFinishers,
        gliders::AccountGliders,
        home::{AccountHomeCats, AccountHomeNodes},
        inventory::AccountInventory,
        luck::AccountLuckVec,
        mailcarriers::AccountMailCarriers,
        mapchests::AccountMapChests,
        masteries::{AccountMasteries, AccountMasteryPoints},
        minis::AccountMinis,
        mounts::{AccountMountSkins, AccountMountTypes},
        outfits::AccountOutfits,
        pvp::AccountPvpHeroes,
        recipes::AccountRecipes,
        skins::AccountSkins,
        titles::AccountTitles,
        worldbosses::AccountWorldBosses,
    },
    Requester,
};

pub mod setup;

#[test]
fn account_achievements() {
    let client = setup::setup();
    let _: AccountAchievements = client.get().unwrap();
}

#[test]
fn account_dailycrafting() {
    let client = setup::setup();
    let _: AccountDailyCrafting = client.get().unwrap();
}

#[test]
fn account_dungeons() {
    let client = setup::setup();
    let _: AccountDungeons = client.get().unwrap();
}

#[test]
fn account_dyes() {
    let client = setup::setup();
    let _: AccountDyes = client.get().unwrap();
}

#[test]
fn account_finishers() {
    let client = setup::setup();
    let _: AccountFinishers = client.get().unwrap();
}

#[test]
fn account_gliders() {
    let client = setup::setup();
    let _: AccountGliders = client.get().unwrap();
}

#[test]
fn account_home_cats() {
    let client = setup::setup();
    let _: AccountHomeCats = client.get().unwrap();
}

#[test]
fn account_home_nodes() {
    let client = setup::setup();
    let _: AccountHomeNodes = client.get().unwrap();
}

#[test]
fn account_inventory() {
    let client = setup::setup();
    let _: AccountInventory = client.get().unwrap();
}

#[test]
fn account_luck() {
    let client = setup::setup();
    let _: AccountLuckVec = client.get().unwrap();
}

#[test]
fn account_mailcarriers() {
    let client = setup::setup();
    let _: AccountMailCarriers = client.get().unwrap();
}

#[test]
fn account_mapchests() {
    let client = setup::setup();
    let _: AccountMapChests = client.get().unwrap();
}

#[test]
fn account_masteries() {
    let client = setup::setup();
    let _: AccountMasteries = client.get().unwrap();
}

#[test]
fn account_mastery_points() {
    let client = setup::setup();
    let _: AccountMasteryPoints = client.get().unwrap();
}

#[test]
fn account_minis() {
    let client = setup::setup();
    let _: AccountMinis = client.get().unwrap();
}

#[test]
fn account_mount_skins() {
    let client = setup::setup();
    let _: AccountMountSkins = client.get().unwrap();
}

#[test]
fn account_mount_types() {
    let client = setup::setup();
    let _: AccountMountTypes = client.get().unwrap();
}

#[test]
fn account_outfits() {
    let client = setup::setup();
    let _: AccountOutfits = client.get().unwrap();
}

#[test]
fn account_pvp_heroes() {
    let client = setup::setup();
    let _: AccountPvpHeroes = client.get().unwrap();
}

#[test]
fn account_recipes() {
    let client = setup::setup();
    let _: AccountRecipes = client.get().unwrap();
}

#[test]
fn account_skins() {
    let client = setup::setup();
    let _: AccountSkins = client.get().unwrap();
}

#[test]
fn account_titles() {
    let client = setup::setup();
    let _: AccountTitles = client.get().unwrap();
}

#[test]
fn account_worldbosses() {
    let client = setup::setup();
    let _: AccountWorldBosses = client.get().unwrap();
}
