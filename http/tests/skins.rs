#![cfg(feature = "blocking")]

use gw2lib::{model::items::skins::Skin, Requester};

pub mod setup;

#[macro_export]
macro_rules! parse_single {
    ($name:ident, $id:expr, $validate:expr) => {
        #[test]
        fn $name() {
            let client = crate::setup::setup();
            let _: gw2lib::model::items::skins::Skin = client.single($id).unwrap();
            let x: gw2lib::model::items::skins::Skin = client.try_single($id).unwrap();
            #[allow(clippy::redundant_closure_call)]
            ($validate)(x);
        }
    };
}

#[macro_export]
macro_rules! check_type {
    ($name:ident) => {
        |x: gw2lib::model::items::skins::Skin| {
            assert_eq!(SkinType::from(x.details), SkinType::$name)
        }
    };
}

#[test]
fn parse_all() {
    let client = setup::setup();
    let _: Vec<Skin> = client.all().unwrap();
    let _: Skin = client.try_single(123).unwrap();
}

mod single {
    use gw2lib::{
        model::items::{
            skins::{Details, GatheringToolsDetails, Skin, SkinType},
            GatheringToolsType,
        },
        Requester,
    };
    parse_single!(armor, 123, check_type!(Armor));
    parse_single!(back, 6344, check_type!(Back));
    parse_single!(gathering, 5656, check_type!(Gathering));
    parse_single!(gathering_fishing, 10721, |x: Skin| assert_eq!(
        x.details,
        Details::Gathering(GatheringToolsDetails {
            _type: GatheringToolsType::Fishing,
        })
    ));
    parse_single!(weapon, 4679, check_type!(Weapon));
}
