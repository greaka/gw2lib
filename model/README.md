# Model

### Breaking Changes

This package mostly follows semver. Patches will be released in the following format:

- `x.*.*` Breaking change that also requires a new version of `gw2lib`
  - updates automatically when updating `gw2lib`
- `*.x.*` Breaking change that is required because of api changes
  - updates automatically with `cargo update`
- `*.*.x` New endpoint/non-breaking change
  - updates automatically with `cargo update`

### Endpoints

An endpoint gets marked as completed once it's mapped out and a small test runs in CI.

Please do help out mapping the API! All you need to do is create the necessary struct and quickly implement 2 traits. If you need help, feel free to reach out.
You don't even need to fork this library to test your struct!

Example commit adding an endpoint: [bcb0bd3](https://github.com/greaka/gw2lib/commit/bcb0bd3e99f135f54fb01d088714ce8471a56d86)

> Last update: 2025/11/03

- achievements
  - [x] achievements
    - [ ] daily
      - [ ] tomorrow
    - [x] groups
    - [x] categories
- authenticated
  - [x] account
    - [x] achievements
    - [x] bank
    - [ ] dailycrafting
    - [ ] dungeons
    - [ ] dyes
    - [ ] finishers
    - [x] inventory
    - [ ] gliders
    - home
      - [ ] cats
      - [ ] nodes
    - [ ] inventory
    - [ ] luck
    - [ ] mailcarries
    - [ ] mapchests
    - [ ] masteries
    - mastery
      - [ ] points
    - [x] materials
    - [ ] minis
    - mounts
      - [ ] skins
      - [ ] types
    - [ ] outfits
    - pvp
      - [ ] heroes
    - [x] raids
    - [ ] recipes
    - [ ] skins
    - [ ] titles
    - [x] wallet
    - [ ] worldbosses
  - characters
    - [x] :id
      - [x] backstory
      - [x] core
      - [x] crafting
      - [x] equipment
      - [ ] heropoints
      - [x] inventory
      - [x] recipes
      - [ ] sab
      - [x] skills
      - [x] specializations
      - [x] training
  - commerce
    - [x] delivery
    - [ ] transactions
  - pvp
    - [ ] stats
    - [ ] games
    - [ ] standings
  - [ ] tokeninfo
- daily rewards
  - [ ] dailycrafting
  - [ ] mapchests
  - [ ] worldbosses
- game mechanics
  - [ ] masteries
  - [ ] mounts
    - [ ] skins
    - [ ] types
  - [ ] outfits
  - [x] pets
  - [ ] professions
  - [ ] races
  - [ ] specializations
  - [ ] skills
  - [ ] traits
  - [ ] legends
- guild
  - guild
    - [ ] :id
    - [ ] permissions
    - [ ] search
    - [ ] upgrades
  - [ ] emblem
- guild authenticated
  - guild
    - :id
      - [ ] log
      - [ ] members
      - [ ] ranks
      - [ ] stash
      - [ ] treasury
      - [ ] teams
      - [ ] upgrades
- home instance
  - home
    - [x] cats
    - [x] nodes
- items
  - [ ] finishers
  - [x] items
  - [x] itemstats
  - [ ] materials
  - pvp
    - [ ] amulets
  - [x] recipes
    - [ ] search
  - [x] skins
- map information
  - [x] continents
  - [x] maps
- Miscellaneous
  - [x] build
  - [ ] colors
  - [ ] currencies
  - [ ] dungeons
  - [ ] files
  - [ ] quaggans
  - [ ] minis
  - [x] raids
  - [ ] titles
  - [x] worlds
- Story
  - backstory
    - [ ] answers
    - [ ] questions
  - [ ] stories
    - [ ] seasons
- sPvP
  - [ ] pvp
    - [ ] ranks
    - [ ] seasons
      - [ ] leaderboards
- trading post
  - commerce
    - [ ] listings
    - [ ] exchange
      - [ ] coins
      - [ ] gems
    - [ ] prices
- world v world
  - [ ] wvw
    - [ ] abilities
    - [ ] matches
    - [ ] objectives
    - [ ] ranks
    - [ ] upgrades
