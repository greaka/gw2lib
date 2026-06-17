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
    - [ ] daily (deprecated)
      - [ ] tomorrow (deprecated)
    - [x] groups
    - [x] categories
- authenticated
  - [x] account
    - [x] achievements
    - [x] bank
    - [x] dailycrafting
    - [x] dungeons
    - [x] dyes
    - [x] finishers
    - [x] gliders
    - home
      - [x] cats
      - [x] nodes
    - [x] inventory
    - [x] luck
    - [x] mailcarriers
    - [x] mapchests
    - [x] masteries
    - mastery
      - [x] points
    - [x] materials
    - [x] minis
    - mounts
      - [x] skins
      - [x] types
    - [x] outfits
    - pvp
      - [x] heroes
    - [x] raids
    - [x] recipes
    - [x] skins
    - [x] titles
    - [x] wallet
    - wizardsvault
      - [x] listings
      - [x] daily
      - [x] weekly
      - [x] special
    - [x] worldbosses
  - characters
    - [x] :id
      - [x] backstory
      - [x] core
      - [x] crafting
      - [x] equipment
      - [x] heropoints
      - [x] inventory
      - [x] recipes
      - [x] sab
      - [x] skills
      - [x] specializations
      - [x] training
  - commerce
    - [x] delivery
    - [x] transactions
  - pvp
    - [x] stats
    - [x] games
    - [x] standings
  - [x] tokeninfo
- daily rewards
  - [x] dailycrafting
  - [x] mapchests
  - [x] worldbosses
- game mechanics
  - [x] masteries
  - [x] mounts
    - [x] skins
    - [x] types
  - [x] outfits
  - [x] pets
  - [x] professions
  - [x] races
  - [x] specializations
  - [x] skills
  - [x] traits
  - [x] legends
- guild
  - guild
    - [x] :id
    - [x] permissions
    - [ ] search
    - [x] upgrades
  - [x] emblem
- guild authenticated
  - guild
    - :id
      - [x] log
      - [x] members
      - [x] ranks
      - [ ] stash
      - [ ] treasury
      - [ ] teams
      - [ ] upgrades
- home instance
  - home
    - [x] cats
    - [x] nodes
- items
  - [x] finishers
  - [x] items
  - [x] itemstats
  - [x] materials
  - pvp
    - [x] amulets
  - [x] recipes
    - [ ] search
  - [x] skins
- map information
  - [x] continents
  - [x] maps
- Miscellaneous
  - [x] build
  - [x] colors
  - [x] currencies
  - [x] dungeons
  - [x] files
  - [x] quaggans
  - [x] minis
  - [x] raids
  - [x] titles
  - [x] worlds
- Story
  - backstory
    - [x] answers
    - [x] questions
  - [x] stories
    - [x] seasons
- sPvP
  - [x] pvp
    - [x] ranks
    - [x] seasons
      - [ ] leaderboards
- trading post
  - commerce
    - [ ] listings
    - [ ] exchange
      - [ ] coins
      - [ ] gems
    - [ ] prices
- world v world
  - [x] wvw
    - [x] abilities
    - [x] matches
    - [x] objectives
    - [x] ranks
    - [x] upgrades
