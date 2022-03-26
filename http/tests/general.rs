use std::sync::Arc;

use gw2api_http::Requester;
use gw2api_model::misc::{build::Build, colors::ColorId};

pub mod setup;

#[test]
fn get() {
    let client = setup::setup();
    let _: Build = client.get().unwrap();
}

#[test]
fn inflight() {
    let client = Arc::new(setup::setup());
    let tclient = client.clone();
    let join = std::thread::spawn(move || {
        let _: Build = tclient.get().unwrap();
        chrono::Utc::now()
    });
    let _: Build = client.get().unwrap();
    let main = chrono::Utc::now();
    let join = join.join().unwrap();
    let diff = (main - join).num_nanoseconds().unwrap().abs();
    dbg!(diff);
    assert!(diff < 10_000);
}

mod cache {
    use std::time::Duration;

    use gw2api_model::misc::colors::Color;

    use super::*;
    #[test]
    fn hit() {
        let client = setup::setup();
        let _: Build = client.get().unwrap();

        let start = chrono::Utc::now();
        let _: Build = client.get().unwrap();
        let end = chrono::Utc::now();
        let cached = (end - start).num_nanoseconds().unwrap();
        assert!(cached < 10_000);
    }

    #[test]
    fn miss() {
        let client = setup::setup();
        let _: Build = client.get().unwrap();

        let start = chrono::Utc::now();
        let _: Vec<ColorId> = client.ids::<Color, ColorId>().unwrap();
        let end = chrono::Utc::now();
        let cached = (end - start).num_nanoseconds().unwrap();
        assert!(cached > 10_000);
    }

    #[test]
    fn forced() {
        let client = setup::setup();
        let _: Build = client.get().unwrap();

        let start = chrono::Utc::now();
        let _: Build = client.forced().get().unwrap();
        let end = chrono::Utc::now();
        let cached = (end - start).num_nanoseconds().unwrap();
        assert!(cached > 10_000);
    }

    #[test]
    fn duration() {
        let client = setup::setup();
        let duration = Duration::from_secs(5);
        let chrono_duration = chrono::Duration::from_std(duration).unwrap();

        let _: Build = client.cached(chrono_duration).get().unwrap();

        // cache hit
        let start = chrono::Utc::now();
        let _: Build = client.get().unwrap();
        let end = chrono::Utc::now();
        let cached = (end - start).num_nanoseconds().unwrap();
        assert!(cached < 10_000);

        std::thread::sleep(duration);

        // cache miss
        let start = chrono::Utc::now();
        let _: Build = client.get().unwrap();
        let end = chrono::Utc::now();
        let cached = (end - start).num_nanoseconds().unwrap();
        assert!(cached > 10_000);
    }
}
