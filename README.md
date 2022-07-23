# GW2 API library [![Build Status](https://img.shields.io/github/workflow/status/greaka/gw2lib/integration_test/master)](https://github.com/greaka/gw2lib/actions/workflows/integration_test.yml) [![Latest Version](https://img.shields.io/crates/v/gw2lib.svg)](https://crates.io/crates/gw2lib) [![](https://img.shields.io/static/v1?label=Get%20Help&message=on%20Discord&style=flat&color=5865f2&labelColor=555&logo=discord&logoColor=fff)](https://discord.gg/bF7Mg38yrx)

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

Missing endpoints are easy to add! Here are two examples:

### Fixed Endpoints

Fixed endpoints are endpoints that do not require or support ids, like the build endpoint.

These endpoints require the use of the `get` method: `client.get().await.unwrap()`

```rust
use serde::{Deserialize, Serialize};

use gw2lib::model::{Endpoint, FixedEndpoint};

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Build {
    pub id: u64,
}

impl Endpoint for Build {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/build";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}

impl FixedEndpoint for Build {}
```

### Bulk Endpoints

Bulk endpoints are endpoints that support ids, like the currencies endpoint.

These endpoints support methods like `ids`, `single`, `many`, `all`, and more.

```rust
use serde::{Deserialize, Serialize};

use gw2lib::model::{BulkEndpoint, Endpoint, EndpointWithId};

pub type CurrencyId = u64;

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Currency {
    pub id: CurrencyId,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub order: u8,
}

impl EndpointWithId for Currency {
    type IdType = CurrencyId;
}
impl Endpoint for Currency {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/currencies";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}

impl BulkEndpoint for Currency {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
```

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
