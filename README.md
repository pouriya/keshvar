# Keshvar
This library contains collection of all sorts of useful information for every country.

[crates.io package](https://crates.io/crates/keshvar)   |   [docs.rs documentation]()   |   [GitHub repository](https://github.com/pouriya/keshvar)

# Features
* Cargo features for all countries, sub-regions, regions, continents, and world regions.  
* For every country it supports:
  * Short and long and unofficial names 
  * Currency code
  * Address format
  * Official and spoken languages
  * Nationality
  * Postal code format
  * Start of the week
  * ...
* Country Subdivisions. (Optional)
* GEO locations for countries and their subdivisions (Optional)
* Translations for countries and subdivisions (Optional)
* [`serde`](serde) integration (Optional)
* [`chrono`](chrono) integration (Optional)
* [`iso_currency`](iso_currency) integration (Optional)

# Quick start
Add this library as a dependency from commandline or do it manually.
#### Commandline
Run bellow command inside root directory of your Cargo project:
```shell
cargo add keshvar 
```
#### Manual
Add `keshvar = "0.1"` under `dependencies` section inside your `Cargo.toml` file.  

Now you're ready to use it inside your Cargo project.

# Examples

## [`Country`](crate::Country) struct
```rust
use keshvar::{Country, CurrencyCode, WeekDay};

let country = Country::try_from("US").unwrap(); // "US" is its alpha2 code
assert_eq!("United States of America", country.iso_short_name);
assert_eq!("The United States of America", country.iso_long_name);
assert!(country.unofficial_name_list.contains(&"United States"));
assert_eq!(CurrencyCode::USD, country.currency_code);
assert_eq!(WeekDay::Sunday, country.start_of_week);
```
## Subdivisions
Enable `subdivisions` feature inside `Cargo.toml` file:
```toml
[dependencies]
keshvar = {version = "0.1", features = ["subdivisions"]}
```
##### Example
```rust
use std::collections::HashMap;
use keshvar::{GEC, Country, Subdivision, SubdivisionType, SubdivisionGeo};

// Load from GEC (Geopolitical Entities and Codes)
let country: Country = GEC::UK.to_country(); // England
// A hashmap containing string subdivision codes as keys and `Subdivision` structs as values:
let subdivisions: HashMap<&str, Subdivision> = country.subdivisions;
let london = subdivisions.get("LND").unwrap();
assert_eq!("London, City of", london.name);
assert_eq!(SubdivisionType::CityCorporation, london.subdivision_type);

// If you enabled `translations` feature:
assert_eq!(Some(&"مدينة لندن"), london.translations.get("ar")); // Arabic

// If you enabled `geo` feature:
let geo: &SubdivisionGeo = london.geo.as_ref().unwrap();
assert_eq!(Some(51.5073509), geo.latitude);
assert_eq!(Some(-0.1277583), geo.longitude);
```
## Translations
Enable `translations` feature inside `Cargo.toml` file:
```toml
[dependencies]
keshvar = {version = "0.1", features = ["translations"]}
```
##### Example
```rust
use std::collections::HashMap;
use keshvar::{Alpha2, Country, find};

// Load from alpha2 code
let country: Country = Alpha2::CN.to_country(); // China
// A hashmap containing languages as keys and translations as values:
let translations: HashMap<&str, &str> = country.translations;
assert_eq!(Some(&"Chine"), translations.get("fr")); // French
assert_eq!(Some(&"Китай"), translations.get("ru")); // Russian

// Find in all translations for name "Ізраїль"
let search_text = "Ізраїль";
let country = find(
    |country: &Country| {
        country.translations.values().collect::<Vec<&&str>>().contains(&&search_text)
    }
).unwrap();
// It's "Ізраїль" is "Israel" in ukrainian language
assert_eq!("Israel", country.iso_short_name);
```
### GEO
Enable `geo` feature inside `Cargo.toml` file:
```toml
[dependencies]
keshvar = {version = "0.1", features = ["geo"]}
```
##### Example
```rust
use keshvar::{IOC, Country, CountryGeo, CountryGeoBounds};

// Load from IOC (International Olympic Committee)
let country: Country = IOC::INA.to_country(); // The Republic of Indonesia (Asia)
let geo: CountryGeo = country.geo;
assert_eq!((-0.789275, 113.921327), (geo.latitude, geo.longitude));
assert_eq!((6.216999899999999, 141.0425), (geo.max_latitude, geo.max_longitude));
assert_eq!((-11.1082999, 94.7351), (geo.min_latitude, geo.min_longitude));
let CountryGeoBounds{northeast, southwest} = geo.bounds;
assert_eq!((6.216999899999999, 141.0425), (northeast.latitude, northeast.longitude));
assert_eq!((-11.1082999, 94.7351), (southwest.latitude, southwest.longitude));
```
## Utility functions
#### find
All `find*` functions at most return one [`Country`](crate::Country) struct.
```rust
use keshvar::{Country, Alpha2, Alpha3, IOC, Region, SubRegion, Continent};
use keshvar::{find_by_iso_short_name, find_by_iso_long_name, find_by_code, find};

let country = find_by_iso_short_name("united states of america").unwrap();
assert_eq!(Some("American"), country.nationality);

let country = find_by_iso_long_name("ukraine").unwrap();
assert_eq!(Alpha2::UA, country.alpha2);
assert_eq!(Some(SubRegion::EasternEurope), country.subregion);

let country = find_by_code(971).unwrap(); // The United Arab Emirates (Asia)
assert_eq!(Alpha3::ARE, country.alpha3);
assert_eq!(Continent::Asia, country.continent);

// Find first match with your own function:
let country = find(
    |country: &Country| country.unofficial_name_list.contains(&"Suisse")
).unwrap();
assert_eq!(Some(IOC::SUI), country.ioc);
assert_eq!(Some(Region::Europe), country.region);
assert!(country.official_language_list.contains(&"de"));
assert!(country.spoken_language_list.contains(&"fr"));
```
#### search
All `search*` functions return zero or more [`Country`](crate::Country) structs.
```rust
use keshvar::{Country, WeekDay, Alpha2, Region, search};

let country_list = search(
    |country: &Country| {
        country.start_of_week == WeekDay::Sunday && country.region == Some(Region::Africa)
    }
);
// Found 4 countries:
assert_eq!(4, country_list.len());
let egypt_country = Alpha2::EG.to_country();
assert!(country_list.contains(&egypt_country));
```
[//]: # (#  A Rust wrapper with some integrations on top of Ruby's [`countries`]&#40;https://github.com/countries/countries&#41; updated data.)
