#![cfg(feature = "blocking")]
#![allow(dead_code)]

use std::sync::Arc;

use gw2lib::{
    Requester,
    model::misc::{build::Build, colors::ColorId},
};

pub mod setup;

#[test]
fn get() {
    let client = setup::setup();
    let _: Build = client.get().unwrap();
}

//#[test]
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
    assert!(dbg!(diff) < 100_000);
}

mod cache {
    use std::time::Duration;

    use gw2lib::model::misc::colors::Color;

    use super::*;
    //#[test]
    fn hit() {
        let client = setup::setup();
        let _: Build = client.get().unwrap();

        let start = chrono::Utc::now();
        let _: Build = client.get().unwrap();
        let end = chrono::Utc::now();
        let cached = (end - start).num_nanoseconds().unwrap();
        assert!(dbg!(cached) < 100_000);
    }

    //#[test]
    fn miss() {
        let client = setup::setup();
        let _: Build = client.get().unwrap();

        let start = chrono::Utc::now();
        let _: Vec<ColorId> = client.ids::<Color>().unwrap();
        let end = chrono::Utc::now();
        let cached = dbg!(end - start).num_nanoseconds().unwrap();
        assert!(dbg!(cached) > 100_000);
    }

    //#[test]
    fn forced() {
        let client = setup::setup();
        let _: Build = client.get().unwrap();

        let start = chrono::Utc::now();
        let _: Build = client.forced().get().unwrap();
        let end = chrono::Utc::now();
        let cached = (end - start).num_nanoseconds().unwrap();
        assert!(dbg!(cached) > 100_000);
    }

    //#[test]
    fn duration() {
        let client = setup::setup();
        let duration = Duration::from_secs(2);
        let chrono_duration = chrono::Duration::from_std(duration).unwrap();

        let _: Build = client.cached(chrono_duration).get().unwrap();

        // cache hit
        let start = chrono::Utc::now();
        let _: Build = client.get().unwrap();
        let end = chrono::Utc::now();
        let cached = (end - start).num_nanoseconds().unwrap();
        assert!(dbg!(cached) < 100_000);

        std::thread::sleep(duration);

        // cache miss
        let start = chrono::Utc::now();
        let _: Build = client.get().unwrap();
        let end = chrono::Utc::now();
        let cached = (end - start).num_nanoseconds().unwrap();
        assert!(dbg!(cached) > 100_000);
    }
}

mod rate_limit {
    use gw2lib::{Requester, rate_limit::BucketRateLimiter};

    use super::*;

    #[test]
    fn hit() {
        // 1 request every 3 seconds
        let rate_limiter = BucketRateLimiter::new(1, 20);
        let client = setup::setup().rate_limiter(rate_limiter.into());
        let client = client.forced();

        let start = chrono::Utc::now();

        // first request
        let _: Build = client.get().unwrap();
        // rate limited request
        let _: Build = client.get().unwrap();

        let end = chrono::Utc::now();
        let limit = (end - start).num_milliseconds();
        assert!(dbg!(limit) > 3_000);
    }
}
