#![cfg(feature = "blocking")]

use gw2lib::{Requester, model::items::recipes::Recipe};

pub mod setup;

#[macro_export]
macro_rules! parse_single {
    ($name:ident, $id:expr, $validate:expr) => {
        #[test]
        fn $name() {
            let client = crate::setup::setup();
            let x: gw2lib::model::items::recipes::Recipe = client.single($id).unwrap();
            #[allow(clippy::redundant_closure_call)]
            ($validate)(x);
        }
    };
}

#[macro_export]
macro_rules! check_type {
    ($name:ident) => {
        |x: gw2lib::model::items::recipes::Recipe| {
            assert_eq!(x._type, gw2lib::model::items::recipes::RecipeType::$name)
        }
    };
}

#[test]
fn all() {
    let client = crate::setup::setup();
    let _: Vec<Recipe> = client.all().unwrap();
}

mod single {
    use gw2lib::Requester;
    parse_single!(refinement_ectoplasm, 7319, check_type!(RefinementEctoplasm));
    parse_single!(insignia, 13598, check_type!(Insignia));
}
