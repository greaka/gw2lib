#![cfg(feature = "blocking")]

use gw2lib::{EndpointError, Requester, model::items::Item};

pub mod setup;

#[macro_export]
macro_rules! parse_single {
    ($name:ident, $id:expr, $validate:expr) => {
        #[test]
        fn $name() {
            let client = $crate::setup::setup();
            let x: gw2lib::model::items::Item = client.single($id).unwrap();
            #[allow(clippy::redundant_closure_call)]
            ($validate)(x);
        }
    };
}

#[macro_export]
macro_rules! check_type {
    ($name:ident) => {
        |x: Item| assert!(ItemType::from(x.details) == ItemType::$name)
    };
}

#[test]
#[ignore]
fn parse_all() {
    let client = crate::setup::setup();
    #[cfg(feature = "redis")]
    let client = {
        use gw2lib::cache::RedisCache;
        let cache = RedisCache::new(redis::Client::open("redis://localhost").unwrap()).into();
        client.cache(cache)
    };
    let res: Result<Vec<Item>, _> = client.all();
    if let Err(EndpointError::InvalidJsonResponse(_)) = res {
        let ids = client.ids::<Item>().unwrap();
        for chunk in ids.chunks(200) {
            let res: Result<Vec<Item>, _> = client.many(chunk.to_vec());
            match res {
                Err(EndpointError::InvalidJsonResponse(_)) => {
                    for &id in chunk {
                        let _: Item = client.single(id).map_err(|e| (id, e)).unwrap();
                    }
                }
                Err(e) => {
                    panic!("{e:?}");
                }
                Ok(_) => {}
            }
        }
    }
}

mod single {
    use gw2lib::{
        Requester,
        model::items::{
            ConsumableDetails, Details, GatheringToolsDetails, GatheringToolsType, Item, ItemType,
            UnlockType, WeaponDetails, WeaponType,
        },
    };
    parse_single!(armor, 80248, check_type!(Armor));
    parse_single!(back, 77474, check_type!(Back));
    parse_single!(bag, 85371, check_type!(Bag));
    parse_single!(consumable, 19993, check_type!(Consumable));
    parse_single!(container, 20316, check_type!(Container));
    parse_single!(gathering, 69478, check_type!(Gathering));
    parse_single!(gizmo, 38506, check_type!(Gizmo));
    parse_single!(mini, 48879, check_type!(MiniPet));
    parse_single!(salvage, 67027, check_type!(Tool));
    parse_single!(trinket, 77958, check_type!(Trinket));
    parse_single!(upgrade, 24691, check_type!(UpgradeComponent));
    parse_single!(weapon, 30699, check_type!(Weapon));
    parse_single!(crafting, 13264, check_type!(CraftingMaterial));
    // doesn't seem to exist anymore
    //    parse_single!(trait_guide, 0,    check_type!(Trait));
    parse_single!(trophy, 18996, check_type!(Trophy));
    parse_single!(relic, 100739, check_type!(Relic));
    parse_single!(key, 82444, check_type!(Key));
    parse_single!(powercore, 95864, check_type!(PowerCore));
    parse_single!(jadetechmodule, 95948, check_type!(JadeTechModule));
    parse_single!(gathering_bait, 95993, |x: Item| assert_eq!(
        x.details,
        Details::Gathering(GatheringToolsDetails {
            _type: GatheringToolsType::Bait,
        })
    ));
    parse_single!(gathering_lure, 98073, |x: Item| assert_eq!(
        x.details,
        Details::Gathering(GatheringToolsDetails {
            _type: GatheringToolsType::Lure,
        })
    ));
    parse_single!(spear, 30691, |x: Item| assert!(matches!(
        x.details,
        Details::Weapon(WeaponDetails {
            _type: WeaponType::Spear,
            ..
        })
    )));
    parse_single!(harpoon_gun, 30697, |x: Item| assert!(matches!(
        x.details,
        Details::Weapon(WeaponDetails {
            _type: WeaponType::HarpoonGun,
            ..
        })
    )));
    parse_single!(magic_door_skin, 105144, |x: Item| {
        assert!(matches!(
            x.details,
            Details::Consumable(ConsumableDetails {
                unlock_type: Some(UnlockType::MagicDoorSkin),
                ..
            })
        ))
    });
}
