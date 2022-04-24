use gw2api_http::Requester;
use gw2api_model::authenticated::characters::Character;

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
