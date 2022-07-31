# GW2 API library [![Build Status](https://img.shields.io/github/workflow/status/greaka/gw2lib/integration_test/master)](https://github.com/greaka/gw2lib/actions/workflows/integration_test.yml) [![Latest Version](https://img.shields.io/crates/v/gw2lib-model.svg)](https://crates.io/crates/gw2lib) [![](https://img.shields.io/static/v1?label=Get%20Help&message=on%20Discord&style=flat&color=5865f2&labelColor=555&logo=discord&logoColor=fff)](https://discord.gg/bF7Mg38yrx)

**gw2lib is an API wrapper for the game Guild Wars 2**

---

Aside from bugs or feature requests that might arise, this crate is considered done!
Mapping out structs is the only work left to do and will be crowd-sourced.
A lack of activity does not mean that it's unmaintained!

## gw2lib in action

### blocking

<details>
<summary>Cargo.toml</summary>

```toml
[dependencies.gw2lib]
version = "1.0.0"
features = ["blocking"]
```
</details>

```rust
use gw2lib::{Client, Requester};
use gw2lib::model::{items::Item, misc::build::Build};

fn main() {
    let client = Client::default();

    let _all_items: Vec<Item> = client.all().unwrap();
    let _current_build: Build = client.get().unwrap();
}
```

### async

<details>
<summary>Cargo.toml</summary>

```toml
[dependencies.gw2lib]
version = "1.0.0"
```
</details>

```rust
use gw2lib::{Client, Requester};
use gw2lib::model::{items::Item, misc::build::Build};

#[tokio::main]
async fn main() {
    let client = Client::default();

    let _all_items: Vec<Item> = client.all().await.unwrap();
    let _current_build: Build = client.get().await.unwrap();
}
```

## Mapped endpoints

You can find the list of already mapped endpoints [here](https://github.com/greaka/gw2lib/blob/master/model/README.md)

Please contribute any endpoints that you define additionally!

## Contributing

Missing endpoints are easy to add! [Here](https://github.com/greaka/gw2lib/commit/bcb0bd3e99f135f54fb01d088714ce8471a56d86) is an example

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in gw2lib by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>
